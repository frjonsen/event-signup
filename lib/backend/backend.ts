import { Construct } from "constructs";
import { HttpApi } from "../gateway/http-gateway";
import { ApiLambda } from "./api-lambda";
import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import * as integrations from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Sentry } from "../sentry";
import { EventTable } from "./event-table";
import { ApiGateway } from "../gateway/api-gateway";
import { EventImageStorage } from "./event-image-storage";

export interface BackendProps {
  gateway: ApiGateway;
  sentry: Sentry;
}

export class Backend extends Construct {
  constructor(scope: Construct, props: BackendProps) {
    super(scope, "Backend");
    const images = new EventImageStorage(this);
    const eventTable = new EventTable(this);
    const apiLambda = new ApiLambda(this, {
      sentry: props.sentry,
      eventTable,
      images,
    });

    props.gateway.httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [agw.HttpMethod.ANY],
      integration: new integrations.HttpLambdaIntegration(
        "ApiIntegration",
        apiLambda,
      ),
    });
  }
}
