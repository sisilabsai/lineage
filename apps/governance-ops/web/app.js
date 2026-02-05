const statusDot = document.getElementById('status-dot');
const statusText = document.getElementById('status-text');
const modeText = document.getElementById('mode-text');

const turnoutEl = document.getElementById('turnout');
const dissentEl = document.getElementById('dissent');
const scarsRoundEl = document.getElementById('scars-round');
const totalDamageEl = document.getElementById('total-damage');
const ledgerTotalEl = document.getElementById('ledger-total');
const roundEl = document.getElementById('round');
const outcomeEl = document.getElementById('outcome');
const membersEl = document.getElementById('members');

const proposalTitleEl = document.getElementById('proposal-title');
const proposalRiskEl = document.getElementById('proposal-risk');
const proposalIdEl = document.getElementById('proposal-id');
const replayFrameEl = document.getElementById('replay-frame');
const votesForEl = document.getElementById('votes-for');
const votesAgainstEl = document.getElementById('votes-against');
const votesAbstainEl = document.getElementById('votes-abstain');

const feedEl = document.getElementById('feed');
const graveyardList = document.getElementById('graveyard-list');

const playToggle = document.getElementById('play-toggle');
const jumpLive = document.getElementById('jump-live');
const speedSelect = document.getElementById('speed');
const scrubSlider = document.getElementById('scrub');
const scrubLabel = document.getElementById('scrub-label');

const adminKeyInput = document.getElementById('admin-key');
const adminKeyStatus = document.getElementById('admin-key-status');
const proposalForm = document.getElementById('proposal-form');
const proposalTitleInput = document.getElementById('proposal-title-input');
const proposalRiskInput = document.getElementById('proposal-risk-input');
const proposalWindowInput = document.getElementById('proposal-window-input');
const voteForm = document.getElementById('vote-form');
const voteProposalIdInput = document.getElementById('vote-proposal-id-input');
const voteChoiceInput = document.getElementById('vote-choice-input');
const voteMemberIdInput = document.getElementById('vote-member-id-input');
const voteMemberNameInput = document.getElementById('vote-member-name-input');
const adminLog = document.getElementById('admin-log');

const dissentChart = new Chart(document.getElementById('dissent-chart').getContext('2d'), {
    type: 'line',
    data: {
        labels: [],
        datasets: [{
            label: 'Dissent %',
            data: [],
            borderColor: '#f59e0b',
            backgroundColor: 'rgba(245, 158, 11, 0.2)',
            tension: 0.3,
            fill: true,
        }]
    },
    options: chartOptions(0, 100),
});

const scarsChart = new Chart(document.getElementById('scars-chart').getContext('2d'), {
    type: 'bar',
    data: {
        labels: [],
        datasets: [{
            label: 'Scars (damage points)',
            data: [],
            backgroundColor: 'rgba(239, 68, 68, 0.6)',
            borderColor: 'rgba(239, 68, 68, 0.9)',
            borderWidth: 1,
        }]
    },
    options: chartOptions(),
});

const energyChart = new Chart(document.getElementById('energy-chart').getContext('2d'), {
    type: 'bar',
    data: {
        labels: [],
        datasets: [{
            label: 'Energy',
            data: [],
            backgroundColor: 'rgba(34, 197, 94, 0.6)',
            borderColor: 'rgba(34, 197, 94, 0.9)',
            borderWidth: 1,
        }]
    },
    options: chartOptions(),
});

const damageChart = new Chart(document.getElementById('damage-chart').getContext('2d'), {
    type: 'bar',
    data: {
        labels: [],
        datasets: [{
            label: 'Damage',
            data: [],
            backgroundColor: 'rgba(239, 68, 68, 0.6)',
            borderColor: 'rgba(239, 68, 68, 0.9)',
            borderWidth: 1,
        }]
    },
    options: chartOptions(),
});

const frames = [];
const metricsHistory = [];
let ledgerEvents = [];
let latestMembers = [];
let graveyardIds = [];
let currentIndex = -1;

let playbackSpeed = 1;
let paused = false;
let playbackTimer = null;
const adminLogEntries = [];

function chartOptions(min, max) {
    return {
        responsive: true,
        scales: {
            y: {
                min,
                max,
                ticks: { color: '#94a3b8' }
            },
            x: { ticks: { color: '#94a3b8' } }
        },
        plugins: { legend: { labels: { color: '#cbd5f5' } } }
    };
}

function formatTime(ms) {
    const date = new Date(ms);
    return date.toLocaleTimeString();
}

function logAdmin(message, level) {
    if (!adminLog) {
        return;
    }
    const timestamp = new Date().toLocaleTimeString();
    const prefix = level === 'error' ? '[error]' : level === 'ok' ? '[ok]' : '[info]';
    adminLogEntries.push(`${timestamp} ${prefix} ${message}`);
    if (adminLogEntries.length > 6) {
        adminLogEntries.shift();
    }
    adminLog.textContent = adminLogEntries.join('\n');
}

function updateAdminKeyStatus() {
    if (!adminKeyStatus || !adminKeyInput) {
        return;
    }
    const key = adminKeyInput.value.trim();
    if (key.length === 0) {
        adminKeyStatus.textContent = 'Key required';
        adminKeyStatus.style.color = '#f59e0b';
    } else {
        adminKeyStatus.textContent = 'Key loaded';
        adminKeyStatus.style.color = '#22c55e';
    }
}

function adminHeaders() {
    const headers = { 'Content-Type': 'application/json' };
    if (adminKeyInput) {
        const key = adminKeyInput.value.trim();
        if (key) {
            headers['X-Admin-Key'] = key;
        }
    }
    return headers;
}

function renderFeed(ledgerTotal) {
    feedEl.innerHTML = '';
    const slice = ledgerEvents.slice(0, ledgerTotal).slice(-30).reverse();
    for (const entry of slice) {
        const item = document.createElement('li');
        item.className = entry.severity;
        const time = document.createElement('span');
        time.className = 'time';
        time.textContent = formatTime(entry.timestamp_ms || entry.timestamp);
        const msg = document.createElement('span');
        msg.textContent = entry.message;
        item.appendChild(time);
        item.appendChild(msg);
        feedEl.appendChild(item);
    }
}

function renderGraveyard(ids) {
    graveyardList.innerHTML = '';
    const slice = ids.slice(-8).reverse();
    for (const id of slice) {
        const item = document.createElement('li');
        item.textContent = id;
        graveyardList.appendChild(item);
    }
    if (slice.length === 0) {
        const item = document.createElement('li');
        item.textContent = 'No tombstones sealed yet.';
        graveyardList.appendChild(item);
    }
}

function renderFrame(index) {
    const frame = frames[index];
    if (!frame) {
        return;
    }

    const metrics = frame.metrics;
    turnoutEl.textContent = `${metrics.turnout_pct.toFixed(1)}%`;
    dissentEl.textContent = `${metrics.dissent_rate_pct.toFixed(1)}%`;
    scarsRoundEl.textContent = metrics.scars_round;
    totalDamageEl.textContent = metrics.total_damage;
    ledgerTotalEl.textContent = metrics.ledger_total;
    roundEl.textContent = metrics.round;
    outcomeEl.textContent = `Outcome: ${metrics.outcome}`;
    membersEl.textContent = `${metrics.members} members`;

    proposalTitleEl.textContent = metrics.title;
    proposalRiskEl.textContent = `Risk: ${metrics.risk}`;
    proposalIdEl.textContent = `ID: ${metrics.proposal_id.slice(0, 10)}...`;

    votesForEl.textContent = metrics.for_votes;
    votesAgainstEl.textContent = metrics.against_votes;
    votesAbstainEl.textContent = metrics.abstain_votes;

    const history = metricsHistory.slice(0, index + 1);
    updateSeries(dissentChart, history.map(item => item.round), history.map(item => item.dissent_rate_pct));
    updateSeries(scarsChart, history.map(item => item.round), history.map(item => item.scars_round));

    const members = frame.members || [];
    updateSeries(energyChart, members.map(member => member.name), members.map(member => member.energy));
    updateSeries(damageChart, members.map(member => member.name), members.map(member => member.damage));

    renderFeed(metrics.ledger_total || ledgerEvents.length);
    renderGraveyard(graveyardIds);

    scrubSlider.value = index;
    scrubLabel.textContent = `Frame ${index + 1}/${frames.length}`;
}

function updateSeries(chart, labels, values) {
    chart.data.labels = labels;
    chart.data.datasets[0].data = values;
    chart.update('none');
}

function startPlayback() {
    if (playbackTimer) {
        clearInterval(playbackTimer);
    }
    playbackTimer = setInterval(() => {
        if (paused) {
            return;
        }
        if (currentIndex < frames.length - 1) {
            currentIndex += 1;
            renderFrame(currentIndex);
        }
    }, 1000 / playbackSpeed);
}

playToggle.addEventListener('click', () => {
    paused = !paused;
    playToggle.textContent = paused ? 'Play' : 'Pause';
    if (!paused && currentIndex === -1 && frames.length > 0) {
        currentIndex = frames.length - 1;
        renderFrame(currentIndex);
    }
});

jumpLive.addEventListener('click', () => {
    if (frames.length === 0) {
        return;
    }
    paused = false;
    playToggle.textContent = 'Pause';
    currentIndex = frames.length - 1;
    renderFrame(currentIndex);
});

speedSelect.addEventListener('change', (event) => {
    playbackSpeed = parseFloat(event.target.value);
    startPlayback();
});

scrubSlider.addEventListener('input', (event) => {
    const index = parseInt(event.target.value, 10);
    if (!Number.isNaN(index)) {
        paused = true;
        playToggle.textContent = 'Play';
        currentIndex = index;
        renderFrame(currentIndex);
    }
});

startPlayback();

if (adminKeyInput) {
    adminKeyInput.value = localStorage.getItem('governance_admin_key') || '';
    updateAdminKeyStatus();
    adminKeyInput.addEventListener('input', () => {
        localStorage.setItem('governance_admin_key', adminKeyInput.value);
        updateAdminKeyStatus();
    });
}

if (proposalForm) {
    proposalForm.addEventListener('submit', async (event) => {
        event.preventDefault();
        if (!adminKeyInput || adminKeyInput.value.trim().length === 0) {
            logAdmin('Admin key is required.', 'error');
            return;
        }
        const title = proposalTitleInput.value.trim();
        if (!title) {
            logAdmin('Proposal title is required.', 'error');
            return;
        }
        const risk = proposalRiskInput.value;
        const windowValue = parseInt(proposalWindowInput.value, 10);
        const payload = { title, risk };
        if (!Number.isNaN(windowValue) && windowValue > 0) {
            payload.voting_window_secs = windowValue;
        }
        try {
            const response = await fetch('/api/admin/proposal', {
                method: 'POST',
                headers: adminHeaders(),
                body: JSON.stringify(payload),
            });
            const result = await response.json().catch(() => ({}));
            if (!response.ok) {
                logAdmin(result.error || 'Proposal failed.', 'error');
                return;
            }
            logAdmin(`Proposal created: ${result.proposal_id}`, 'ok');
            proposalTitleInput.value = '';
        } catch (err) {
            logAdmin(`Proposal failed: ${err.message}`, 'error');
        }
    });
}

if (voteForm) {
    voteForm.addEventListener('submit', async (event) => {
        event.preventDefault();
        if (!adminKeyInput || adminKeyInput.value.trim().length === 0) {
            logAdmin('Admin key is required.', 'error');
            return;
        }
        const proposalId = voteProposalIdInput.value.trim();
        if (!proposalId) {
            logAdmin('Proposal ID is required.', 'error');
            return;
        }
        const payload = {
            proposal_id: proposalId,
            choice: voteChoiceInput.value,
        };
        const memberId = voteMemberIdInput.value.trim();
        const memberName = voteMemberNameInput.value.trim();
        if (memberId) {
            payload.member_id = memberId;
        }
        if (memberName) {
            payload.member_name = memberName;
        }
        try {
            const response = await fetch('/api/admin/vote', {
                method: 'POST',
                headers: adminHeaders(),
                body: JSON.stringify(payload),
            });
            const result = await response.json().catch(() => ({}));
            if (!response.ok) {
                logAdmin(result.error || 'Vote failed.', 'error');
                return;
            }
            logAdmin(
                `Vote cast: ${result.receipt.member_id} -> ${result.receipt.choice}`,
                'ok'
            );
            voteProposalIdInput.value = '';
        } catch (err) {
            logAdmin(`Vote failed: ${err.message}`, 'error');
        }
    });
}

const ws = new WebSocket(`ws://${location.host}/ws`);

ws.onopen = () => {
    statusDot.classList.add('online');
    statusText.textContent = 'Connected';
};

ws.onclose = () => {
    statusDot.classList.remove('online');
    statusText.textContent = 'Disconnected';
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);

    if (data.type === 'status') {
        modeText.textContent = `Mode: ${data.mode || 'live'}`;
    }

    if (data.type === 'snapshot' || data.type === 'resync') {
        if (Array.isArray(data.ledger_events)) {
            ledgerEvents = data.ledger_events;
        }
        if (Array.isArray(data.members)) {
            latestMembers = data.members;
        }
        if (Array.isArray(data.graveyard_ids)) {
            graveyardIds = data.graveyard_ids;
        }
        if (Array.isArray(data.metrics_history) && data.metrics_history.length > 0) {
            metricsHistory.length = 0;
            frames.length = 0;
            data.metrics_history.forEach((metrics) => {
                metricsHistory.push(metrics);
                frames.push({
                    metrics,
                    members: latestMembers,
                });
            });
            scrubSlider.max = Math.max(frames.length - 1, 0);
            currentIndex = frames.length - 1;
            if (currentIndex >= 0) {
                renderFrame(currentIndex);
            }
            return;
        }
        if (data.metrics) {
            metricsHistory.push(data.metrics);
            frames.push({
                metrics: data.metrics,
                members: latestMembers,
            });
            scrubSlider.max = Math.max(frames.length - 1, 0);
            currentIndex = frames.length - 1;
            renderFrame(currentIndex);
        }
        return;
    }

    if (data.type === 'metrics') {
        metricsHistory.push(data);
        frames.push({
            metrics: data,
            members: latestMembers,
        });

        scrubSlider.max = Math.max(frames.length - 1, 0);

        if (!paused && (currentIndex >= frames.length - 2 || currentIndex === -1)) {
            currentIndex = frames.length - 1;
            renderFrame(currentIndex);
        }
    }

    if (data.type === 'members') {
        latestMembers = data.members || [];
        if (!paused && currentIndex === frames.length - 1) {
            updateSeries(energyChart, latestMembers.map(m => m.name), latestMembers.map(m => m.energy));
            updateSeries(damageChart, latestMembers.map(m => m.name), latestMembers.map(m => m.damage));
        }
    }

    if (data.type === 'ledger_event') {
        ledgerEvents.push(data);
    }

    if (data.type === 'graveyard_event') {
        if (!graveyardIds.includes(data.id)) {
            graveyardIds.push(data.id);
        }
        renderGraveyard(graveyardIds);
    }

    if (data.type === 'replay') {
        replayFrameEl.textContent = `Replay: ${data.frame}/${data.total_frames}`;
    }
};
