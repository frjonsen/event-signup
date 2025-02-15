import * as cdk from "aws-cdk-lib";
import * as cf from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as route53Targets from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";
import { HttpApi } from "./http-gateway";
import { CloudfrontStack } from "../stacks/cloudfront-stack";
import { Domain } from "../domain";

export interface CloudfrontProps {
  httpApi: HttpApi;
  cloudfront: CloudfrontStack;
  domain: Domain;
}

export class Cloudfront extends Construct {
  constructor(scope: Construct, props: CloudfrontProps) {
    super(scope, "Cloudfront");

    // httpApi.url also contains the protocol, so we need to extract the domain
    const origin = cdk.Fn.join(".", [
      props.httpApi.httpApi.httpApiId,
      "execute-api",
      cdk.Fn.ref("AWS::Region"),
      "amazonaws.com",
    ]);

    const originPolicy = new cf.OriginRequestPolicy(this, "OriginPolicy", {
      headerBehavior: cf.OriginRequestHeaderBehavior.allowList(
        "access-control-request-method",
        "origin",
        "user-agent",
        "sentry-trace",
      ),
      queryStringBehavior: cf.OriginRequestQueryStringBehavior.all(),
    });

    const apiOrigin = new origins.HttpOrigin(origin);
    const distribution = new cf.Distribution(this, "Distribution", {
      certificate: props.cloudfront.certificate,
      domainNames: [Domain.EVENTS_SIGNUP_DOMAIN],
      priceClass: cf.PriceClass.PRICE_CLASS_100,
      httpVersion: cf.HttpVersion.HTTP2_AND_3,
      defaultBehavior: {
        viewerProtocolPolicy: cf.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        allowedMethods: cf.AllowedMethods.ALLOW_ALL,
        originRequestPolicy: originPolicy,
        origin: apiOrigin,
      },
    });

    distribution.addBehavior("/api/*", apiOrigin, {
      cachePolicy: cf.CachePolicy.CACHING_DISABLED,
      originRequestPolicy: originPolicy,
    });

    const target = route53.RecordTarget.fromAlias(
      new route53Targets.CloudFrontTarget(distribution),
    );
    new route53.ARecord(this, "ARecord", {
      target,
      recordName: Domain.EVENTS_SIGNUP_DOMAIN,
      zone: props.domain.zone,
    });
    new route53.AaaaRecord(this, "AaaaRecord", {
      target,
      recordName: Domain.EVENTS_SIGNUP_DOMAIN,
      zone: props.domain.zone,
    });
  }
}
