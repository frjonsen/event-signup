import { Construct } from "constructs";
import { HttpApi } from "./http-gateway";
import { Domain } from "../domain";
import { CloudfrontStack } from "../stacks/cloudfront-stack";
import { Cloudfront } from "./cloudfront";
import { Frontend } from "../frontend/hosting/frontend";

export interface ApiGatewayProps {
  cloudfront: CloudfrontStack;
  domain: Domain;
  frontend: Frontend;
}

export class ApiGateway extends Construct {
  public readonly httpApi: HttpApi;
  public readonly cloudFront: Cloudfront;
  constructor(scope: Construct, props: ApiGatewayProps) {
    super(scope, "ApiGateway");

    this.httpApi = new HttpApi(this);

    this.cloudFront = new Cloudfront(this, {
      cloudfront: props.cloudfront,
      httpApi: this.httpApi,
      domain: props.domain,
      frontend: props.frontend,
    });
  }
}
