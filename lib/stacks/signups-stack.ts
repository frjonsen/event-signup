import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { EventTable } from "../event-table";
import { Domain } from "../domain";
import { HttpApi } from "../gateway/http-gateway";
import { CloudfrontStack } from "./cloudfront-stack";
import { ApiGateway } from "../gateway/api-gateway";
import { Sentry } from "../sentry";
import { Backend } from "../backend/backend";

export interface SignupsStackProps extends cdk.StackProps {
  cloudfront: CloudfrontStack;
}

export class SignupsStack extends cdk.Stack {
  constructor(scope: Construct, props: SignupsStackProps) {
    super(scope, "SignupStack", {
      ...props,
      crossRegionReferences: true,
      env: {
        region: "eu-north-1",
      },
    });

    const sentry = new Sentry(this);
    const zone = new Domain(this);
    const gateway = new ApiGateway(this, {
      domain: zone,
      cloudfront: props.cloudfront,
    });
    const backend = new Backend(this, { httpApi: gateway.httpApi, sentry });
    const eventTable = new EventTable(this);
  }
}
