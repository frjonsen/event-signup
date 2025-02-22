import { Construct } from "constructs";
import { ApiLambda } from "./api-lambda";
import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import * as integrations from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Sentry } from "../sentry";
import { EventTable } from "./event-table";
import { ApiGateway } from "../gateway/api-gateway";
import { EventImageStorage } from "./event-image-storage";
import { Authentication } from "../authentication/authentication";
import { CognitoUserPoolsAuthorizer } from "aws-cdk-lib/aws-apigateway";
import { HttpUserPoolAuthorizer } from "aws-cdk-lib/aws-apigatewayv2-authorizers";

export interface BackendProps {
  gateway: ApiGateway;
  sentry: Sentry;
  authentication: Authentication;
}

export class Backend extends Construct {
  constructor(scope: Construct, props: BackendProps) {
    super(scope, "Backend");
    const images = new EventImageStorage(this);
    const eventTable = new EventTable(this);
    const apiLambda = new ApiLambda(this, "ApiLambda", {
      sentry: props.sentry,
      eventTable,
      images,
    });
    const imageUploadLambda = new ApiLambda(this, "ImageUploadLambda", {
      sentry: props.sentry,
      eventTable,
      images,
      memory: 2048,
    });

    const adminAuthorizer = new HttpUserPoolAuthorizer(
      "EventCreatorAuthorizer",
      props.authentication.userPool,
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
