import { Construct } from "constructs";
import { UserPool } from "./user-pool";
import * as cognito from "aws-cdk-lib/aws-cognito";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { Domain } from "../domain";

export class Authentication extends Construct {
  public readonly userPool: UserPool;
  public readonly client: cognito.UserPoolClient;
  constructor(scope: Construct) {
    super(scope, "Authentication");
    this.userPool = new UserPool(this);
    // Will be generated with a random value, and be replaced manually after deployment
    const clientSecret = new secretsmanager.Secret(this, "ClientSecret");
    const clientId = ssm.StringParameter.valueForStringParameter(
      this,
      "/events/auth/google/clientId",
    );
    new cognito.UserPoolIdentityProviderGoogle(this, "GoogleProvider", {
      clientId,
      clientSecretValue: clientSecret.secretValue,
      userPool: this.userPool,
      scopes: ["email"],
      attributeMapping: {
        email: cognito.ProviderAttribute.GOOGLE_EMAIL,
      },
    });

    this.client = new cognito.UserPoolClient(this, "UserPoolClient", {
      userPool: this.userPool,
      authFlows: {},
      oAuth: {
        callbackUrls: [`https://${Domain.EVENTS_DOMAIN}`],
        scopes: [cognito.OAuthScope.EMAIL, cognito.OAuthScope.PROFILE],
      },
    });
  }
}
