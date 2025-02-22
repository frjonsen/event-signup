import * as s3 from "aws-cdk-lib/aws-s3";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

export class EventsSiteBucket extends s3.Bucket {
  constructor(scope: Construct) {
    super(scope, "EventsSiteBucket", {
      autoDeleteObjects: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });
  }
}
