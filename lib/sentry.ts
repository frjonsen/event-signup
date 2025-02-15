import { Construct } from "constructs";
import * as ssm from "aws-cdk-lib/aws-ssm";

export class Sentry extends Construct {
  public readonly backendDsn: ssm.IStringParameter;
  public readonly frontendDsn: ssm.IStringParameter;
  constructor(scope: Construct) {
    super(scope, "Sentry");

    this.backendDsn = ssm.StringParameter.fromStringParameterName(
      this,
      "BackendDsn",
      "/events/sentry/backend",
    );

    this.frontendDsn = ssm.StringParameter.fromStringParameterName(
      this,
      "FrontendDsn",
      "/events/sentry/frontend",
    );
  }
}
