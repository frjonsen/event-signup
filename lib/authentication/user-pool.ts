import * as cognito from "aws-cdk-lib/aws-cognito";
import { Construct } from "constructs";

export class UserPool extends cognito.UserPool {
  constructor(scope: Construct) {
    super(scope, "UserPool", {
      selfSignUpEnabled: false,
    });

    new cognito.UserPoolDomain(this, "Domain", {
      userPool: this,
      cognitoDomain: {
        domainPrefix: "events-signup",
      },
    });
  }
}
