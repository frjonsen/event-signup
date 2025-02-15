import * as cdk from "aws-cdk-lib";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import { Construct } from "constructs";
import { Domain } from "../domain";

export class CloudfrontStack extends cdk.Stack {
  public readonly certificate: acm.Certificate;
  constructor(scope: Construct) {
    super(scope, "CloudfrontStack", {
      crossRegionReferences: true,
      env: {
        region: "us-east-1",
      },
    });

    const zone = new Domain(this);

    this.certificate = new acm.Certificate(this, "Certificate", {
      domainName: Domain.EVENTS_SIGNUP_DOMAIN,
      validation: acm.CertificateValidation.fromDns(zone.zone),
      keyAlgorithm: acm.KeyAlgorithm.EC_PRIME256V1,
    });
  }
}
