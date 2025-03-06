import * as cognito from "aws-cdk-lib/aws-cognito";
import { TransitionDefaultMinimumObjectSize } from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";

export class UserPool extends cognito.UserPool {
  public static readonly CONTENT_CREATORS_GROUP_NAME = "ContentCreators";

  constructor(scope: Construct) {
    super(scope, "UserPool", {
      selfSignUpEnabled: false,
      signInPolicy: {},
      signInAliases: {
        username: false,
        email: true,
      },
      standardAttributes: {
        email: { required: true, mutable: false },
      },
      accountRecovery: cognito.AccountRecovery.EMAIL_ONLY,
    });

    new cognito.UserPoolDomain(this, "Domain", {
      userPool: this,
      cognitoDomain: {
        domainPrefix: "events-signup",
      },
    });

    new cognito.UserPoolGroup(this, "ContentCreators", {
      userPool: this,
      groupName: UserPool.CONTENT_CREATORS_GROUP_NAME,
      description: "Users who can create content",
    });
  }
}
