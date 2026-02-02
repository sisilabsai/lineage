// WebSocket Dashboard Client
class LineageDashboard {
    constructor() {
        this.ws = null;
        this.priceHistory = { BTC_USD: [], ETH_USD: [] };
        this.agentHistory = {};
        this.totalTrades = 0;
        this.updateCount = 0;
        this.charts = {};
        this.maxHistoryPoints = 50;

        this.initializeCharts();
        this.connectWebSocket();
        this.setupEventListeners();
    }

    connectWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const url = `${protocol}//127.0.0.1:9001`;

        try {
            this.ws = new WebSocket(url);

            this.ws.onopen = () => {
                console.log('WebSocket connected');
                this.updateConnectionStatus(true);
            };

            this.ws.onmessage = (event) => {
                this.handleMessage(JSON.parse(event.data));
            };

            this.ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.updateConnectionStatus(false);
            };

            this.ws.onclose = () => {
                console.log('WebSocket disconnected');
                this.updateConnectionStatus(false);
                // Attempt to reconnect after 3 seconds
                setTimeout(() => this.connectWebSocket(), 3000);
            };
        } catch (error) {
            console.error('Failed to create WebSocket:', error);
            this.updateConnectionStatus(false);
            setTimeout(() => this.connectWebSocket(), 3000);
        }
    }

    handleMessage(data) {
        if (data.type === 'market') {
            this.updateMarketData(data);
        } else if (data.type === 'agent') {
            this.updateAgentData(data);
        }
    }

    updateMarketData(data) {
        const { symbol, price, timestamp } = data;
        const key = symbol.replace('-', '_');

        // Update price history
        if (!this.priceHistory[key]) {
            this.priceHistory[key] = [];
        }

        this.priceHistory[key].push({
            time: new Date(timestamp * 1000).toLocaleTimeString(),
            price: price
        });

        // Keep only last N points
        if (this.priceHistory[key].length > this.maxHistoryPoints) {
            this.priceHistory[key].shift();
        }

        // Update UI
        if (symbol === 'BTC-USD') {
            this.updatePriceDisplay('btc-price', 'btc-change', price);
        } else if (symbol === 'ETH-USD') {
            this.updatePriceDisplay('eth-price', 'eth-change', price);
        } else if (symbol === 'SOL-USD') {
            this.updatePriceDisplay('sol-price', 'sol-change', price);
        } else if (symbol === 'ADA-USD') {
            this.updatePriceDisplay('ada-price', 'ada-change', price);
        } else if (symbol === 'DOT-USD') {
            this.updatePriceDisplay('dot-price', 'dot-change', price);
        }

        // Update charts
        this.updatePriceChart();
        this.incrementUpdateCount();
    }

    updateAgentData(data) {
        const { agent_name, capital, trades, win_rate, scars, action } = data;

        // Initialize agent history if needed
        if (!this.agentHistory[agent_name]) {
            this.agentHistory[agent_name] = {
                history: [],
                lastUpdate: 0
            };
        }

        // Store history
        this.agentHistory[agent_name].history.push({
            time: new Date().toLocaleTimeString(),
            capital: capital,
            trades: trades,
            win_rate: win_rate
        });

        if (this.agentHistory[agent_name].history.length > this.maxHistoryPoints) {
            this.agentHistory[agent_name].history.shift();
        }

        this.agentHistory[agent_name].lastUpdate = Date.now();

        // Update agents table
        this.updateAgentsTable();

        // Update capital chart
        this.updateCapitalChart();

        // Log trade if action indicates a trade occurred
        if (action && action.includes('trade')) {
            const isWin = action.includes('won') || win_rate > 50;
            this.logTrade(agent_name, action, isWin);
            this.totalTrades++;
            document.getElementById('total-trades').textContent = this.totalTrades;
        }

        this.incrementUpdateCount();
    }

    updatePriceDisplay(priceId, changeId, price) {
        const priceEl = document.getElementById(priceId);
        const changeEl = document.getElementById(changeId);

        const oldPrice = parseFloat(priceEl.textContent.replace(/[$,]/g, '')) || price;
        const change = ((price - oldPrice) / oldPrice * 100).toFixed(2);

        priceEl.textContent = `$${price.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
        priceEl.className = change >= 0 ? 'positive' : 'negative';

        changeEl.textContent = `${change >= 0 ? '↑' : '↓'} ${Math.abs(change)}%`;
        changeEl.className = change >= 0 ? 'positive' : 'negative';
    }

    updateAgentsTable() {
        const tbody = document.getElementById('agents-tbody');

        if (Object.keys(this.agentHistory).length === 0) return;

        tbody.innerHTML = Object.entries(this.agentHistory).map(([name, data]) => {
            const latest = data.history[data.history.length - 1] || {};
            const isActive = Date.now() - data.lastUpdate < 15000;

            return `
                <tr class="agent-row">
                    <td class="agent-name">
                        <span>${this.getAgentIcon(name)} ${name}</span>
                    </td>
                    <td><strong>${latest.capital?.toLocaleString('en-US', { minimumFractionDigits: 2 }) || '—'}</strong></td>
                    <td>${latest.trades || 0}</td>
                    <td><span class="${latest.win_rate >= 50 ? 'positive' : 'negative'}">${latest.win_rate?.toFixed(1) || '—'}%</span></td>
                    <td>${data.history.length > 1 ? `${(data.history.length - 1)}` : 0}</td>
                    <td>
                        <span class="status-badge ${isActive ? 'status-active' : 'status-resting'}">
                            ${isActive ? '🟢 Active' : '⏸️ Resting'}
                        </span>
                    </td>
                </tr>
            `;
        }).join('');

        // Update average win rate
        const avgWinRate = Object.values(this.agentHistory).reduce((sum, data) => {
            const latest = data.history[data.history.length - 1];
            return sum + (latest?.win_rate || 0);
        }, 0) / Object.keys(this.agentHistory).length;

        document.getElementById('avg-win-rate').textContent = avgWinRate.toFixed(1) + '%';
    }

    logTrade(agent, action, isWin) {
        const feed = document.getElementById('trades-feed');

        // Clear placeholder if needed
        if (feed.querySelector('[style*="text-align: center"]')) {
            feed.innerHTML = '';
        }

        const tradeEl = document.createElement('div');
        tradeEl.className = `trade-event ${isWin ? 'trade-win' : 'trade-loss'}`;

        const time = new Date().toLocaleTimeString();
        const emoji = isWin ? '✅' : '❌';

        tradeEl.innerHTML = `
            <div style="display: flex; justify-content: space-between;">
                <strong>${emoji} ${agent}</strong>
                <span class="trade-time">${time}</span>
            </div>
            <div style="margin-top: 4px; font-size: 12px;">${action}</div>
        `;

        feed.insertBefore(tradeEl, feed.firstChild);

        // Keep only last 20 trades
        while (feed.children.length > 20) {
            feed.removeChild(feed.lastChild);
        }
    }

    getAgentIcon(name) {
        const icons = {
            'Momentum': '⚡',
            'Conservative': '🛡️',
            'Balanced': '⚖️',
            'Aggressive': '🔥',
            'Cautious': '🐢'
        };
        return icons[name] || '🤖';
    }

    initializeCharts() {
        // Price Chart
        const priceCtx = document.getElementById('priceChart').getContext('2d');
        this.charts.price = new Chart(priceCtx, {
            type: 'line',
            data: {
                labels: [],
                datasets: [
                    {
                        label: 'BTC-USD',
                        data: [],
                        borderColor: '#f59e0b',
                        backgroundColor: 'rgba(245, 158, 11, 0.1)',
                        borderWidth: 2,
                        tension: 0.4,
                        fill: true,
                        pointRadius: 0,
                        pointHoverRadius: 6
                    },
                    {
                        label: 'ETH-USD',
                        data: [],
                        borderColor: '#667eea',
                        backgroundColor: 'rgba(102, 126, 234, 0.1)',
                        borderWidth: 2,
                        tension: 0.4,
                        fill: true,
                        pointRadius: 0,
                        pointHoverRadius: 6
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        labels: { color: '#e0e0e0', font: { size: 12 } }
                    }
                },
                scales: {
                    y: {
                        ticks: { color: '#666' },
                        grid: { color: 'rgba(255, 255, 255, 0.05)' }
                    },
                    x: {
                        ticks: { color: '#666' },
                        grid: { color: 'rgba(255, 255, 255, 0.05)' }
                    }
                }
            }
        });

        // Capital Chart
        const capitalCtx = document.getElementById('capitalChart').getContext('2d');
        this.charts.capital = new Chart(capitalCtx, {
            type: 'bar',
            data: {
                labels: [],
                datasets: []
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        labels: { color: '#e0e0e0', font: { size: 12 } }
                    }
                },
                scales: {
                    y: {
                        ticks: { color: '#666' },
                        grid: { color: 'rgba(255, 255, 255, 0.05)' }
                    },
                    x: {
                        ticks: { color: '#666' },
                        grid: { color: 'rgba(255, 255, 255, 0.05)' }
                    }
                }
            }
        });
    }

    updatePriceChart() {
        const labels = this.priceHistory.BTC_USD.map(p => p.time);
        const btcData = this.priceHistory.BTC_USD.map(p => p.price);
        const ethData = this.priceHistory.ETH_USD.map(p => p.price);

        this.charts.price.data.labels = labels;
        this.charts.price.data.datasets[0].data = btcData;
        this.charts.price.data.datasets[1].data = ethData;
        this.charts.price.update('none');
    }

    updateCapitalChart() {
        const labels = Object.keys(this.agentHistory);
        const colors = ['#f59e0b', '#667eea', '#22c55e', '#ef4444', '#ec4899'];

        const datasets = labels.map((agent, index) => {
            const history = this.agentHistory[agent].history;
            const latest = history[history.length - 1];

            return {
                label: agent,
                data: [latest?.capital || 0],
                backgroundColor: colors[index % colors.length]
            };
        });

        this.charts.capital.data.labels = labels;
        this.charts.capital.data.datasets = datasets;
        this.charts.capital.update('none');
    }

    updateConnectionStatus(connected) {
        const statusDot = document.getElementById('ws-status');
        const statusText = document.getElementById('ws-text');

        if (connected) {
            statusDot.classList.remove('offline');
            statusText.textContent = 'Connected';
            statusText.style.color = '#22c55e';
        } else {
            statusDot.classList.add('offline');
            statusText.textContent = 'Disconnected';
            statusText.style.color = '#ef4444';
        }
    }

    incrementUpdateCount() {
        this.updateCount++;
        document.getElementById('update-count').textContent = this.updateCount;
    }

    setupEventListeners() {
        // Auto-refresh stats every 5 seconds
        setInterval(() => {
            this.updateAgentsTable();
        }, 5000);
    }
}

// Initialize dashboard when page loads
document.addEventListener('DOMContentLoaded', () => {
    new LineageDashboard();
});
