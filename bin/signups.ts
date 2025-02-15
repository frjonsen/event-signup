#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { SignupsStack } from "../lib/stacks/signups-stack";
import { CloudfrontStack } from "../lib/stacks/cloudfront-stack";

const app = new cdk.App();
const cloudfront = new CloudfrontStack(app);
new SignupsStack(app, {
  cloudfront,
});
