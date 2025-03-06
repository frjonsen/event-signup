import { Construct } from "constructs";
import { ApiLambda } from "./api-lambda";
import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import * as integrations from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Sentry } from "../sentry";
import { EventTable } from "./event-table";
import { ApiGateway } from "../gateway/api-gateway";
import { EventImageStorage } from "./event-image-storage";
import { Authentication } from "../authentication/authentication";
import { HttpUserPoolAuthorizer } from "aws-cdk-lib/aws-apigatewayv2-authorizers";

export interface BackendProps {
  gateway: ApiGateway;
  sentry: Sentry;
  authentication: Authentication;
  database: EventTable;
}

export class Backend extends Construct {
  constructor(scope: Construct, props: BackendProps) {
    super(scope, "Backend");
    const images = new EventImageStorage(this);
    props.gateway.cloudFront.addS3Origin("/static/*", images);
    const apiLambda = new ApiLambda(this, "ApiLambda", {
      sentry: props.sentry,
      eventTable: props.database,
      images,
    });
    const imageUploadLambda = new ApiLambda(this, "ImageUploadLambda", {
      sentry: props.sentry,
      eventTable: props.database,
      images,
      memory: 2048,
    });

    const adminAuthorizer = new HttpUserPoolAuthorizer(
      "EventCreatorAuthorizer",
      props.authentication.userPool,
      {
        userPoolClients: [props.authentication.client],
      },
    );

    const apiIntegration = new integrations.HttpLambdaIntegration(
      "ApiIntegration",
      apiLambda,
    );
    props.gateway.httpApi.addRoutes({
      path: "/api/public/{proxy+}",
      methods: [agw.HttpMethod.ANY],
      integration: apiIntegration,
    });

    props.gateway.httpApi.addRoutes({
      path: "/api/admin/{proxy+}",
      methods: [agw.HttpMethod.ANY],
      integration: apiIntegration,
      authorizer: adminAuthorizer,
    });

    props.gateway.httpApi.addRoutes({
      path: "/api/admin/event/{eventId}/images",
      methods: [agw.HttpMethod.POST],
      integration: new integrations.HttpLambdaIntegration(
        "ImageUploadIntegration",
        imageUploadLambda,
      ),
      authorizer: adminAuthorizer,
    });
  }
}
