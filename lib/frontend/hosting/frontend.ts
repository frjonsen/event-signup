import { Construct } from "constructs";
import { ApiGateway } from "../../gateway/api-gateway";
import { EventsSiteBucket } from "./events-site-bucket";
import { EventsSiteDeploymentAsset } from "./events-site-deployment-asset";

export class Frontend extends Construct {
  public readonly bucket: EventsSiteBucket;
  constructor(scope: Construct) {
    super(scope, "Frontend");

    this.bucket = new EventsSiteBucket(this);
    const deployment = new EventsSiteDeploymentAsset(this, {
      bucket: this.bucket,
    });
  }
}
