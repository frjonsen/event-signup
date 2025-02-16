import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import { Construct } from "constructs";

export class HttpApi extends agw.HttpApi {
  constructor(scope: Construct) {
    super(scope, "Gateway");
  }
}
