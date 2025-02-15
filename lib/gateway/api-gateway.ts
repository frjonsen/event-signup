import { Construct } from "constructs";
import { HttpApi } from "./http-gateway";
import { Domain } from "../domain";
import { CloudfrontStack } from "../stacks/cloudfront-stack";
import { Cloudfront } from "./cloudfront";

export interface ApiGatewayProps {
  cloudfront: CloudfrontStack;
  domain: Domain;
}

export class ApiGateway extends Construct {
  public readonly httpApi: HttpApi;
  constructor(scope: Construct, props: ApiGatewayProps) {
    super(scope, "ApiGateway");

    this.httpApi = new HttpApi(this);

    const cloudfront = new Cloudfront(this, {
      cloudfront: props.cloudfront,
      httpApi: this.httpApi,
      domain: props.domain,
    });
  }
}
