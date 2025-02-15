import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import { Construct } from "constructs";

export class HttpApi extends Construct {
  public readonly httpApi: agw.HttpApi;

  constructor(scope: Construct) {
    super(scope, "Gateway");

    this.httpApi = this.constructGateway();
  }

  private constructGateway(): agw.HttpApi {
    return new agw.HttpApi(this, "Api");
  }
}
