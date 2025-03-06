import * as s3 from "aws-cdk-lib/aws-s3";
import * as cdk from "aws-cdk-lib";
import * as cf from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as route53Targets from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";
import { HttpApi } from "./http-gateway";
import { CloudfrontStack } from "../stacks/cloudfront-stack";
import { Domain } from "../domain";
import { Frontend } from "../frontend/hosting/frontend";

export interface CloudfrontProps {
  httpApi: HttpApi;
  cloudfront: CloudfrontStack;
  domain: Domain;
  frontend: Frontend;
}

export class Cloudfront extends Construct {
  public readonly distribution: cf.Distribution;
  constructor(scope: Construct, props: CloudfrontProps) {
    super(scope, "Cloudfront");

    // httpApi.url also contains the protocol, so we need to extract the domain
    const origin = cdk.Fn.join(".", [
      props.httpApi.httpApiId,
      "execute-api",
      cdk.Fn.ref("AWS::Region"),
      "amazonaws.com",
    ]);

    // const originPolicy = new cf.OriginRequestPolicy(this, "OriginPolicy", {
    //   headerBehavior: cf.OriginRequestHeaderBehavior.allowList(
    //     "access-control-request-method",
    //     "origin",
    //     "user-agent",
    //     "sentry-trace",
    //   ),
    //   queryStringBehavior: cf.OriginRequestQueryStringBehavior.all(),
    // });

    const apiOrigin = new origins.HttpOrigin(origin);
    const frontendOrigin = origins.S3BucketOrigin.withOriginAccessControl(
      props.frontend.bucket,
    );
    this.distribution = new cf.Distribution(this, "Distribution", {
      certificate: props.cloudfront.certificate,
      domainNames: [Domain.EVENTS_DOMAIN],
      priceClass: cf.PriceClass.PRICE_CLASS_100,
      httpVersion: cf.HttpVersion.HTTP2_AND_3,
      defaultBehavior: {
        viewerProtocolPolicy: cf.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        allowedMethods: cf.AllowedMethods.ALLOW_ALL,
        originRequestPolicy:
          cf.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
        origin: frontendOrigin,
        cachePolicy: cf.CachePolicy.CACHING_DISABLED,
        edgeLambdas: [
          {
            eventType: cf.LambdaEdgeEventType.VIEWER_REQUEST,
            functionVersion: props.cloudfront.redirect.currentVersion,
          },
        ],
      },
      defaultRootObject: "index.html",
    });

    this.distribution.addBehavior("/assets/*", frontendOrigin, {
      originRequestPolicy: cf.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
      allowedMethods: cf.AllowedMethods.ALLOW_GET_HEAD,
      viewerProtocolPolicy: cf.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
    });

    this.distribution.addBehavior("/api/*", apiOrigin, {
      cachePolicy: cf.CachePolicy.CACHING_DISABLED,
      originRequestPolicy: cf.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
      allowedMethods: cf.AllowedMethods.ALLOW_ALL,
      viewerProtocolPolicy: cf.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
    });

    const target = route53.RecordTarget.fromAlias(
      new route53Targets.CloudFrontTarget(this.distribution),
    );
    new route53.ARecord(this, "ARecord", {
      target,
      recordName: Domain.EVENTS_DOMAIN,
      zone: props.domain.zone,
    });
    new route53.AaaaRecord(this, "AaaaRecord", {
      target,
      recordName: Domain.EVENTS_DOMAIN,
      zone: props.domain.zone,
    });
  }

  public addS3Origin(path: string, bucket: s3.IBucket) {
    const origin = origins.S3BucketOrigin.withOriginAccessControl(bucket, {});
    this.distribution.addBehavior(path, origin, {
      allowedMethods: cf.AllowedMethods.ALLOW_GET_HEAD,
      viewerProtocolPolicy: cf.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
    });
  }
}
