import { Construct } from "constructs";
import { HttpApi } from "../gateway/http-gateway";
import { ApiLambda } from "./api-lambda";
import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import * as integrations from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Sentry } from "../sentry";
import { EventTable } from "../event-table";

export interface BackendProps {
  httpApi: HttpApi;
  sentry: Sentry;
  eventTable: EventTable;
}

export class Backend extends Construct {
  constructor(scope: Construct, props: BackendProps) {
    super(scope, "Backend");
    const apiLambda = new ApiLambda(this, {
      sentry: props.sentry,
      eventTable: props.eventTable,
    });

    props.httpApi.httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [agw.HttpMethod.ANY],
      integration: new integrations.HttpLambdaIntegration(
        "ApiIntegration",
        apiLambda,
      ),
    });
  }
}
