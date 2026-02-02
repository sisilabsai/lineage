/// Finance module visualization for arena results
/// Generates charts and graphs in multiple formats (ASCII, HTML, PNG)

/// Visualization output formats
#[derive(Debug, Clone, Copy)]
pub enum ChartFormat {
    Ascii,      // Terminal-friendly ASCII art
    Html,       // Interactive HTML charts
    Csv,        // Data export for external tools
}

/// Arena visualization builder
pub struct ArenaVisualizer {
    final_rankings: Vec<(String, f64, f64, f64)>, // (name, capital, roi, win_rate)
    market_prices: Vec<(u32, f64)>,               // (round, price)
}

impl ArenaVisualizer {
    pub fn new() -> Self {
        Self {
            final_rankings: Vec::new(),
            market_prices: Vec::new(),
        }
    }

    pub fn with_rankings(mut self, rankings: Vec<(String, f64, f64, f64)>) -> Self {
        self.final_rankings = rankings;
        self
    }

    pub fn with_prices(mut self, prices: Vec<(u32, f64)>) -> Self {
        self.market_prices = prices;
        self
    }

    /// Generate ASCII capital growth chart
    pub fn ascii_capital_chart(&self) -> String {
        if self.final_rankings.is_empty() {
            return "No data available".to_string();
        }

        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║           CAPITAL GROWTH COMPARISON (FINAL)                ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        let max_capital = self.final_rankings.iter().map(|(_, c, _, _)| c).fold(0.0, |acc, &x| if x > acc { x } else { acc });
        let scale = 50.0 / max_capital;

        for (rank, (name, capital, roi, _)) in self.final_rankings.iter().enumerate() {
            let bar_width = (*capital * scale) as usize;
            let bar = "█".repeat(bar_width);

            output.push_str(&format!(
                "#{:<2} {:<15} │ {} {:>7.0} (+{:>3.0}%)\n",
                rank + 1,
                name,
                bar,
                capital,
                roi
            ));
        }

        output.push_str("\n");
        output
    }

    /// Generate ASCII ROI comparison chart
    pub fn ascii_roi_chart(&self) -> String {
        if self.final_rankings.is_empty() {
            return "No data available".to_string();
        }

        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              RETURN ON INVESTMENT (ROI) COMPARISON          ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        let max_roi = self.final_rankings.iter().map(|(_, _, r, _)| r).fold(0.0, |acc, &x| if x > acc { x } else { acc });
        let min_roi = self.final_rankings.iter().map(|(_, _, r, _)| r).fold(0.0, |acc, &x| if x < acc { x } else { acc });
        let range = (max_roi - min_roi).max(1.0);
        let scale = 40.0 / range;
        let offset = min_roi;

        for (name, _, roi, _) in self.final_rankings.iter() {
            let bar_width = ((roi - offset) * scale) as usize;
            let bar = if *roi >= 0.0 {
                format!("{}→", "═".repeat(bar_width))
            } else {
                format!("←{}", "═".repeat(bar_width))
            };

            output.push_str(&format!("{:<15} │ {:>6.1}% {}\n", name, roi, bar));
        }

        output.push_str("\n");
        output
    }

    /// Generate ASCII win rate chart
    pub fn ascii_win_rate_chart(&self) -> String {
        if self.final_rankings.is_empty() {
            return "No data available".to_string();
        }

        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    WIN RATE COMPARISON                      ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        for (name, _, _, win_rate) in self.final_rankings.iter() {
            let bar_width = (*win_rate / 2.0) as usize;
            let bar = "▰".repeat(bar_width) + &"▱".repeat(50 - bar_width);
            output.push_str(&format!("{:<15} │ {} {:>5.1}%\n", name, bar, win_rate));
        }

        output.push_str("\n");
        output
    }

    /// Generate ASCII market price trend
    pub fn ascii_price_trend(&self) -> String {
        if self.market_prices.is_empty() {
            return "No market data available".to_string();
        }

        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                   BTC-USD PRICE TREND                       ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        let min_price = self.market_prices.iter().map(|(_, p)| p).fold(f64::MAX, |acc, &x| if x < acc { x } else { acc });
        let max_price = self.market_prices.iter().map(|(_, p)| p).fold(0.0, |acc, &x| if x > acc { x } else { acc });
        let price_range = max_price - min_price;
        let scale = 20.0 / price_range;

        output.push_str(&format!("  {:.0} ┤\n", max_price));
        output.push_str("         │\n");

        for (round, price) in &self.market_prices {
            let height = ((*price - min_price) * scale) as usize;
            let symbol = if height > 15 { "█" } else if height > 10 { "▆" } else { "▂" };
            output.push_str(&format!(
                "R{:>2}    │ {}\n",
                round,
                symbol.repeat((height / 2).max(1))
            ));
        }

        output.push_str(&format!("  {:.0} └", min_price));
        output.push_str(&"─".repeat(60));
        output.push_str("\n");
        output.push_str(&format!("          Price range: ${:.2} → ${:.2}\n\n", min_price, max_price));

        output
    }

    /// Generate ASCII summary statistics table
    pub fn ascii_summary_table(&self) -> String {
        let mut output = String::new();
        output.push_str("╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                   SUMMARY STATISTICS                        ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        output.push_str("┌─────────────┬──────────┬────────┬──────────┬──────────┐\n");
        output.push_str("│ Agent       │ Capital  │ ROI    │ Win Rate │ Ranking  │\n");
        output.push_str("├─────────────┼──────────┼────────┼──────────┼──────────┤\n");

        for (rank, (name, capital, roi, win_rate)) in self.final_rankings.iter().enumerate() {
            output.push_str(&format!(
                "│ {:<11} │ ${:<7.0} │ {:>5.1}% │ {:>7.1}% │    #{:<4} │\n",
                name,
                capital,
                roi,
                win_rate,
                rank + 1
            ));
        }

        output.push_str("└─────────────┴──────────┴────────┴──────────┴──────────┘\n\n");

        output
    }

    /// Generate HTML chart (requires plotly)
    pub fn generate_html_charts(&self, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("    <title>Lineage Finance Arena Results</title>\n");
        html.push_str("    <script src=\"https://cdn.plot.ly/plotly-latest.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n");
        html.push_str("        .container { max-width: 1200px; margin: 0 auto; }\n");
        html.push_str("        .chart { background: white; margin: 20px 0; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        h1 { color: #333; border-bottom: 3px solid #0066cc; padding-bottom: 10px; }\n");
        html.push_str("        h2 { color: #0066cc; margin-top: 30px; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <h1>🏦 Lineage Finance - Arena Simulation Results</h1>\n");

        // Capital growth chart
        html.push_str("        <div class=\"chart\">\n");
        html.push_str("            <h2>📊 Capital Growth</h2>\n");
        html.push_str("            <div id=\"capitalChart\" style=\"width:100%;height:400px;\"></div>\n");
        html.push_str("        </div>\n");

        // ROI comparison chart
        html.push_str("        <div class=\"chart\">\n");
        html.push_str("            <h2>📈 Return on Investment</h2>\n");
        html.push_str("            <div id=\"roiChart\" style=\"width:100%;height:400px;\"></div>\n");
        html.push_str("        </div>\n");

        // Win rate chart
        html.push_str("        <div class=\"chart\">\n");
        html.push_str("            <h2>🎯 Win Rate Distribution</h2>\n");
        html.push_str("            <div id=\"winRateChart\" style=\"width:100%;height:400px;\"></div>\n");
        html.push_str("        </div>\n");

        // Price trend chart
        if !self.market_prices.is_empty() {
            html.push_str("        <div class=\"chart\">\n");
            html.push_str("            <h2>💹 Market Price Trend</h2>\n");
            html.push_str("            <div id=\"priceChart\" style=\"width:100%;height:400px;\"></div>\n");
            html.push_str("        </div>\n");
        }

        html.push_str("        <script>\n");
        html.push_str(&self.generate_plotly_js());
        html.push_str("        </script>\n");
        html.push_str("    </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        // Write to file
        std::fs::write(filename, &html)?;
        Ok(format!("HTML charts saved to {}", filename))
    }

    fn generate_plotly_js(&self) -> String {
        let mut js = String::new();

        // Extract data
        let names: Vec<_> = self.final_rankings.iter().map(|(n, _, _, _)| n.clone()).collect();
        let capitals: Vec<_> = self.final_rankings.iter().map(|(_, c, _, _)| c).collect();
        let rois: Vec<_> = self.final_rankings.iter().map(|(_, _, r, _)| r).collect();
        let win_rates: Vec<_> = self.final_rankings.iter().map(|(_, _, _, w)| w).collect();

        // Capital chart
        js.push_str("            var capitalData = [{\n");
        js.push_str("                x: ");
        js.push_str(&format!("{:?}", names));
        js.push_str(",\n");
        js.push_str("                y: ");
        js.push_str(&format!("{:?}", capitals));
        js.push_str(",\n");
        js.push_str("                type: 'bar',\n");
        js.push_str("                marker: {color: '#0066cc'}\n");
        js.push_str("            }];\n");
        js.push_str("            var capitalLayout = {title: 'Final Capital by Agent', yaxis: {title: 'Capital ($)'}};\n");
        js.push_str("            Plotly.newPlot('capitalChart', capitalData, capitalLayout);\n\n");

        // ROI chart
        js.push_str("            var roiData = [{\n");
        js.push_str("                x: ");
        js.push_str(&format!("{:?}", names));
        js.push_str(",\n");
        js.push_str("                y: ");
        js.push_str(&format!("{:?}", rois));
        js.push_str(",\n");
        js.push_str("                type: 'bar',\n");
        js.push_str("                marker: {color: '#00cc66'}\n");
        js.push_str("            }];\n");
        js.push_str("            var roiLayout = {title: 'ROI by Agent', yaxis: {title: 'ROI (%)'}};\n");
        js.push_str("            Plotly.newPlot('roiChart', roiData, roiLayout);\n\n");

        // Win rate chart
        js.push_str("            var winRateData = [{\n");
        js.push_str("                x: ");
        js.push_str(&format!("{:?}", names));
        js.push_str(",\n");
        js.push_str("                y: ");
        js.push_str(&format!("{:?}", win_rates));
        js.push_str(",\n");
        js.push_str("                type: 'bar',\n");
        js.push_str("                marker: {color: '#ff9900'}\n");
        js.push_str("            }];\n");
        js.push_str("            var winRateLayout = {title: 'Win Rate by Agent', yaxis: {title: 'Win Rate (%)'}};\n");
        js.push_str("            Plotly.newPlot('winRateChart', winRateData, winRateLayout);\n\n");

        // Price trend if available
        if !self.market_prices.is_empty() {
            let rounds: Vec<_> = self.market_prices.iter().map(|(r, _)| r).collect();
            let prices: Vec<_> = self.market_prices.iter().map(|(_, p)| p).collect();

            js.push_str("            var priceData = [{\n");
            js.push_str("                x: ");
            js.push_str(&format!("{:?}", rounds));
            js.push_str(",\n");
            js.push_str("                y: ");
            js.push_str(&format!("{:?}", prices));
            js.push_str(",\n");
            js.push_str("                type: 'scatter',\n");
            js.push_str("                mode: 'lines+markers',\n");
            js.push_str("                line: {color: '#cc0066', width: 2}\n");
            js.push_str("            }];\n");
            js.push_str("            var priceLayout = {title: 'BTC-USD Price Trend', xaxis: {title: 'Round'}, yaxis: {title: 'Price ($)'}};\n");
            js.push_str("            Plotly.newPlot('priceChart', priceData, priceLayout);\n");
        }

        js
    }

    /// Generate CSV data export
    pub fn generate_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("Rank,Agent,Final Capital,ROI %,Win Rate %\n");

        for (rank, (name, capital, roi, win_rate)) in self.final_rankings.iter().enumerate() {
            csv.push_str(&format!(
                "{},{},{:.2},{:.2},{:.2}\n",
                rank + 1,
                name,
                capital,
                roi,
                win_rate
            ));
        }

        csv
    }

    /// Display all ASCII charts to console
    pub fn display_all(&self) {
        println!("{}", self.ascii_capital_chart());
        println!("{}", self.ascii_roi_chart());
        println!("{}", self.ascii_win_rate_chart());
        if !self.market_prices.is_empty() {
            println!("{}", self.ascii_price_trend());
        }
        println!("{}", self.ascii_summary_table());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_creation() {
        let visualizer = ArenaVisualizer::new(vec![]);
        assert!(visualizer.final_rankings.is_empty());
    }

    #[test]
    fn test_ascii_charts_not_empty() {
        let visualizer = ArenaVisualizer::new(vec![])
            .with_rankings(vec![
                ("Agent1".to_string(), 105000.0, 5.0, 60.0),
                ("Agent2".to_string(), 100000.0, 0.0, 0.0),
            ]);

        let capital_chart = visualizer.ascii_capital_chart();
        assert!(!capital_chart.is_empty());
        assert!(capital_chart.contains("Agent1"));

        let roi_chart = visualizer.ascii_roi_chart();
        assert!(!roi_chart.is_empty());
        assert!(roi_chart.contains("5.0"));
    }

    #[test]
    fn test_csv_generation() {
        let visualizer = ArenaVisualizer::new(vec![])
            .with_rankings(vec![("Agent1".to_string(), 105000.0, 5.0, 60.0)]);

        let csv = visualizer.generate_csv();
        assert!(csv.contains("Agent1"));
        assert!(csv.contains("105000"));
    }
}
