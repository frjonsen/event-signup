import * as cf from "aws-cdk-lib/aws-cloudfront";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";

export class IndexRedirect extends cf.experimental.EdgeFunction {
  constructor(scope: Construct) {
    super(scope, "IndexRedirect", {
      code: lambda.Code
        .fromInline(`exports.handler = (event, context, callback) => {
          const request = event.Records[0].cf.request;

          if (!request.uri.startsWith("/vite.svg") && !request.uri.startsWith("/assets")) {
              request.uri = "/index.html";
          }
          callback(null, request);
        };
      `),
      handler: "index.handler",
      runtime: lambda.Runtime.NODEJS_22_X,
    });
  }
}
