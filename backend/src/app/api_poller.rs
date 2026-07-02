/*
 * Copyright (C) 2026 Lordseriouspig
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::app::models::api::api_state::SharedApiState;
use crate::app::models::api::stats::{StatsConf, StatsResp};
use color_eyre::Result;
use color_eyre::eyre::eyre;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub async fn spawn_poller(state: SharedApiState, client: reqwest::Client, stats_cfg: StatsConf) {
    tracing::info!("Starting API Poller");
    let stats_cfg = Arc::new(stats_cfg);
    let mut interval = tokio::time::interval(Duration::from_secs(300));

    loop {
        interval.tick().await;
        if let Err(e) = async {
            let data = fetch_stats(&client, stats_cfg.clone()).await?;

            let mut s = state.write().await;
            s.stats = Some(data);
            s.last_updated = Some(Instant::now());

            Ok::<(), color_eyre::Report>(())
        }
        .await
        {
            tracing::warn!(error=?e, "Error fetching stats");
        }
    }
}

async fn fetch_stats(client: &reqwest::Client, cfg: Arc<StatsConf>) -> Result<StatsResp> {
    let username = match &cfg.username {
        Some(u) => u,
        None => return Err(eyre!("No username provided")),
    };
    let url = format!("{}/api/v1/users/{}/stats", cfg.base_url, username);
    let mut req = client.get(url);
    if let Some(api_key) = &cfg.api_key {
        req = req.header("Authorization", format!("Bearer {}", api_key));
    } else {
        tracing::debug!("No Hackatime API key provided");
    };
    let res = req.send().await?;
    Ok(res.json::<StatsResp>().await?)
}
