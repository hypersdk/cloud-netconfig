
// SPDX-License-Identifier: LGPL-3.0-or-later

use anyhow::Result;
use futures::stream::TryStreamExt;
use rtnetlink::new_connection;
use rtnetlink::packet_route::rule::RuleAttribute;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct RoutingPolicyRule {
    pub from: Option<String>,
    pub to: Option<String>,
    pub table: u32,
}

pub async fn routing_policy_rule_add(rule: &RoutingPolicyRule) -> Result<()> {
    let links = super::acquire_links().await?;
    if links.links_by_mac.len() < 2 {
        return Ok(());
    }

    if rule_exists(rule).await? {
        return Ok(());
    }

    let (connection, handle, _) = new_connection()?;
    tokio::spawn(connection);

    let mut rule_request = handle.rule().add();

    if let Some(ref from) = rule.from {
        let ip: IpAddr = from.parse()?;
        rule_request = rule_request.source_prefix(ip, 32);
    }

    if let Some(ref to) = rule.to {
        let ip: IpAddr = to.parse()?;
        rule_request = rule_request.destination_prefix(ip, 32);
    }

    rule_request = rule_request.table(rule.table);

    let result = rule_request.execute().await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_str = format!("{}", e);
            if err_str.contains("File exists") || err_str.contains("EEXIST") {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to add routing policy rule: {}", e))
            }
        }
    }
}

pub async fn routing_policy_rule_remove(rule: &RoutingPolicyRule) -> Result<()> {
    let (connection, handle, _) = new_connection()?;
    tokio::spawn(connection);

    let mut rule_request = handle.rule().del();

    if let Some(ref from) = rule.from {
        let ip: IpAddr = from.parse()?;
        rule_request = rule_request.source_prefix(ip, 32);
    }

    if let Some(ref to) = rule.to {
        let ip: IpAddr = to.parse()?;
        rule_request = rule_request.destination_prefix(ip, 32);
    }

    rule_request = rule_request.table(rule.table);
    rule_request.execute().await?;

    Ok(())
}

async fn rule_exists(rule: &RoutingPolicyRule) -> Result<bool> {
    let (connection, handle, _) = new_connection()?;
    tokio::spawn(connection);

    let mut rule_stream = handle.rule().get(rtnetlink::IpVersion::V4).execute();

    while let Some(rule_msg) = rule_stream.try_next().await? {
        let table = rule_msg.header.table as u32;
        if table != rule.table {
            continue;
        }

        let mut src_ip: Option<String> = None;
        let mut dst_ip: Option<String> = None;

        for attr in &rule_msg.attributes {
            match attr {
                RuleAttribute::Source(ip) => src_ip = Some(ip.to_string()),
                RuleAttribute::Destination(ip) => dst_ip = Some(ip.to_string()),
                _ => {}
            }
        }

        let from_matches = match (&rule.from, &src_ip) {
            (Some(from), Some(src)) => from == src,
            (None, None) => true,
            _ => false,
        };

        let to_matches = match (&rule.to, &dst_ip) {
            (Some(to), Some(dst)) => to == dst,
            (None, None) => true,
            _ => false,
        };

        if from_matches && to_matches {
            return Ok(true);
        }
    }

    Ok(false)
}
