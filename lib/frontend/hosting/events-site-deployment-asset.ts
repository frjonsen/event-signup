import { Construct } from "constructs";
import * as s3deploy from "aws-cdk-lib/aws-s3-deployment";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as cdk from "aws-cdk-lib";
import { execSync } from "child_process";
import * as fs from "fs";

export interface EventsSiteDeploymentAssetProps {
  bucket: s3.Bucket;
}

export class EventsSiteDeploymentAsset extends Construct {
  public readonly deployment: s3deploy.BucketDeployment;
  constructor(scope: Construct, props: EventsSiteDeploymentAssetProps) {
    super(scope, "EventsSiteDeploymentAsset");

    const source = this.bundle();
    this.deployment = new s3deploy.BucketDeployment(this, "Deployment", {
      sources: [source],
      destinationBucket: props.bucket,
    });
  }

  bundle(): s3deploy.ISource {
    const path = "lib/frontend/events-site";
    return s3deploy.Source.asset(path, {
      assetHashType: cdk.AssetHashType.SOURCE,
      bundling: {
        command: ["pnpm s3-bundle", "cp -r /asset-input/dist/* /asset-output"],
        image: cdk.DockerImage.fromRegistry("node:23"),
        outputType: cdk.BundlingOutput.NOT_ARCHIVED,
        local: {
          tryBundle(outputDir: string): boolean {
            const buildDir = `${process.env.INIT_CWD}/${path}`;
            execSync(`pnpm s3-bundle > /dev/null`, {
              stdio: "inherit",
              cwd: buildDir,
              env: {
                ...process.env,
                OUTPUT_DIR: outputDir,
              },
            });

            fs.cpSync(`${buildDir}/dist`, outputDir, { recursive: true });
            return true;
          },
        },
      },
    });
  }
}
