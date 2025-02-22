import * as route53 from "aws-cdk-lib/aws-route53";
import { Construct } from "constructs";

export class Domain extends Construct {
  public static readonly EVENTS_DOMAIN = "events.jonsen.se";
  public readonly zone: route53.IHostedZone;

  constructor(scope: Construct) {
    super(scope, "Domain");

    this.zone = route53.HostedZone.fromHostedZoneAttributes(this, "Zone", {
      hostedZoneId: "Z1ZVM1WEI5TDOH",
      zoneName: "jonsen.se",
    });
  }
}
