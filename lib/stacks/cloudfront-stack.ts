import * as cdk from "aws-cdk-lib";
import * as cf from "aws-cdk-lib/aws-cloudfront";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import { Construct } from "constructs";
import { Domain } from "../domain";
import { IndexRedirect } from "../frontend/hosting/index-redirect";

export class CloudfrontStack extends cdk.Stack {
  public readonly certificate: acm.Certificate;
  public readonly redirect: cf.experimental.EdgeFunction;
  constructor(scope: Construct) {
    super(scope, "CloudfrontStack", {
      crossRegionReferences: true,
      env: {
        region: "us-east-1",
      },
    });

    const zone = new Domain(this);
    this.redirect = new IndexRedirect(this);

    this.certificate = new acm.Certificate(this, "Certificate", {
      domainName: Domain.EVENTS_DOMAIN,
      validation: acm.CertificateValidation.fromDns(zone.zone),
      keyAlgorithm: acm.KeyAlgorithm.EC_PRIME256V1,
    });
  }
}
