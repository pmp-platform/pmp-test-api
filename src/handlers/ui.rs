use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

/// UI endpoint handler
/// Returns a beautiful HTML page that displays the info endpoint data
pub async fn ui_handler() -> Response {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PMP Test API - System Information</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
            color: #333;
        }

        .container {
            max-width: 1400px;
            margin: 0 auto;
        }

        header {
            text-align: center;
            color: white;
            margin-bottom: 30px;
            padding: 30px 20px;
        }

        header h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
        }

        header p {
            font-size: 1.1rem;
            opacity: 0.95;
        }

        .tabs-container {
            background: white;
            border-radius: 16px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
            overflow: hidden;
        }

        .tabs {
            display: flex;
            background: #f8f9fa;
            border-bottom: 2px solid #e9ecef;
            overflow-x: auto;
            flex-wrap: wrap;
        }

        .tab {
            padding: 18px 28px;
            cursor: pointer;
            border: none;
            background: transparent;
            font-size: 1rem;
            font-weight: 600;
            color: #6c757d;
            transition: all 0.3s ease;
            border-bottom: 3px solid transparent;
            white-space: nowrap;
            position: relative;
        }

        .tab:hover {
            background: rgba(102, 126, 234, 0.1);
            color: #667eea;
        }

        .tab.active {
            color: #667eea;
            border-bottom-color: #667eea;
            background: white;
        }

        .tab-badge {
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 2px 8px;
            border-radius: 12px;
            font-size: 0.75rem;
            margin-left: 8px;
            font-weight: 700;
        }

        .tab.active .tab-badge {
            background: #764ba2;
        }

        .tab-content {
            display: none;
            padding: 30px;
            animation: fadeIn 0.3s ease;
        }

        .tab-content.active {
            display: block;
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }

        .loading {
            text-align: center;
            padding: 60px 20px;
            background: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
        }

        .loading-spinner {
            border: 4px solid #f3f3f3;
            border-top: 4px solid #667eea;
            border-radius: 50%;
            width: 50px;
            height: 50px;
            animation: spin 1s linear infinite;
            margin: 0 auto 20px;
        }

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }

        .error {
            background: #fee;
            color: #c33;
            padding: 20px;
            border-radius: 8px;
            margin: 20px 0;
            border-left: 4px solid #c33;
        }

        .section-header {
            color: #495057;
            margin-bottom: 25px;
            font-size: 1.3rem;
            border-bottom: 2px solid #e9ecef;
            padding-bottom: 12px;
            font-weight: 700;
        }

        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 20px;
        }

        .status-badge {
            display: inline-block;
            padding: 6px 14px;
            border-radius: 20px;
            font-size: 0.85rem;
            font-weight: 700;
            margin-left: 10px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }

        .status-success {
            background: linear-gradient(135deg, #06d6a0 0%, #05c798 100%);
            color: white;
        }

        .status-error {
            background: linear-gradient(135deg, #ef476f 0%, #e63956 100%);
            color: white;
        }

        .env-var {
            display: flex;
            justify-content: space-between;
            padding: 12px 16px;
            background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
            border-radius: 8px;
            margin-bottom: 10px;
            font-family: 'Courier New', monospace;
            font-size: 0.9rem;
            border-left: 4px solid #667eea;
            transition: all 0.2s ease;
        }

        .env-var:hover {
            transform: translateX(4px);
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .env-key {
            font-weight: 700;
            color: #495057;
            word-break: break-all;
            flex: 0 0 40%;
        }

        .env-value {
            color: #6c757d;
            word-break: break-all;
            text-align: right;
            flex: 1;
            margin-left: 20px;
        }

        .check-item {
            background: linear-gradient(135deg, #ffffff 0%, #f8f9fa 100%);
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 20px;
            border: 1px solid #e9ecef;
            transition: all 0.3s ease;
        }

        .check-item:hover {
            transform: translateY(-4px);
            box-shadow: 0 8px 20px rgba(0, 0, 0, 0.12);
        }

        .check-item.sql {
            border-left: 5px solid #3b82f6;
        }

        .check-item.nosql {
            border-left: 5px solid #10b981;
        }

        .check-item.http {
            border-left: 5px solid #f59e0b;
        }

        .check-item.s3 {
            border-left: 5px solid #ec4899;
        }

        .check-item.memorydb {
            border-left: 5px solid #8b5cf6;
        }

        .check-item.secrets {
            border-left: 5px solid #ef4444;
        }

        .check-item.dynamodb {
            border-left: 5px solid #06b6d4;
        }

        .check-item.bedrock {
            border-left: 5px solid #f97316;
        }

        .check-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }

        .check-title {
            font-weight: 700;
            color: #495057;
            font-size: 1.1rem;
        }

        .check-details {
            display: grid;
            gap: 10px;
            font-size: 0.9rem;
            color: #6c757d;
        }

        .detail-row {
            display: flex;
            justify-content: space-between;
            padding: 8px 12px;
            background: rgba(255, 255, 255, 0.6);
            border-radius: 6px;
        }

        .detail-label {
            font-weight: 600;
            color: #495057;
        }

        .detail-value {
            font-family: 'Courier New', monospace;
            text-align: right;
            word-break: break-all;
            margin-left: 15px;
        }

        .empty-state {
            text-align: center;
            padding: 60px 20px;
            color: #6c757d;
            font-style: italic;
            font-size: 1.1rem;
        }

        .empty-state-icon {
            font-size: 4rem;
            margin-bottom: 20px;
            opacity: 0.3;
        }

        .list-items {
            margin-top: 10px;
            padding-left: 20px;
        }

        .list-items li {
            padding: 6px 0;
            color: #6c757d;
        }

        .stats-summary {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 15px;
            margin-bottom: 30px;
        }

        .stat-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 12px;
            text-align: center;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.15);
        }

        .stat-number {
            font-size: 2rem;
            font-weight: 700;
            margin-bottom: 5px;
        }

        .stat-label {
            font-size: 0.9rem;
            opacity: 0.9;
        }

        @media (max-width: 768px) {
            header h1 {
                font-size: 1.8rem;
            }

            .tabs {
                flex-direction: column;
            }

            .tab {
                text-align: left;
                padding: 15px 20px;
            }

            .grid {
                grid-template-columns: 1fr;
            }

            .env-var {
                flex-direction: column;
            }

            .env-key {
                flex: 1;
                margin-bottom: 5px;
            }

            .env-value {
                text-align: left;
                margin-left: 0;
            }

            .tab-content {
                padding: 20px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🚀 PMP Test API</h1>
            <p>Platform Monitoring & Connectivity Health Dashboard</p>
        </header>

        <div id="content">
            <div class="loading">
                <div class="loading-spinner"></div>
                <p>Loading system information...</p>
            </div>
        </div>
    </div>

    <script>
        let systemData = null;

        async function loadData() {
            try {
                const response = await fetch('/_/info');

                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                systemData = await response.json();
                renderTabbedUI(systemData);
            } catch (error) {
                document.getElementById('content').innerHTML = `
                    <div class="error">
                        <h2>Error Loading Data</h2>
                        <p>${error.message}</p>
                    </div>
                `;
            }
        }

        function renderTabbedUI(data) {
            const tabs = [];
            const tabContents = [];

            // Count items for badges
            const counts = {
                environments: data.environments ? Object.keys(data.environments).length : 0,
                http: data.http ? Object.keys(data.http).length : 0,
                sql: data.sql ? Object.keys(data.sql).length : 0,
                nosql: data.nosql ? Object.keys(data.nosql).length : 0,
                s3: data.s3 ? Object.keys(data.s3).length : 0,
                memorydb: data.memorydb ? Object.keys(data.memorydb).length : 0,
                secrets_manager: data.secrets_manager ? Object.keys(data.secrets_manager).length : 0,
                dynamodb: data.dynamodb ? Object.keys(data.dynamodb).length : 0,
                bedrock: data.bedrock ? Object.keys(data.bedrock).length : 0,
            };

            // Environment Variables Tab
            if (counts.environments > 0) {
                tabs.push({ id: 'environments', label: '🌍 Environments', count: counts.environments });
                tabContents.push({
                    id: 'environments',
                    content: renderEnvironments(data.environments)
                });
            }

            // HTTP APIs Tab
            if (counts.http > 0) {
                tabs.push({ id: 'http', label: '🌐 HTTP APIs', count: counts.http });
                tabContents.push({
                    id: 'http',
                    content: renderHttpApis(data.http)
                });
            }

            // Databases Tab
            const dbCount = counts.sql + counts.nosql + counts.dynamodb;
            if (dbCount > 0) {
                tabs.push({ id: 'databases', label: '🗄️ Databases', count: dbCount });
                tabContents.push({
                    id: 'databases',
                    content: renderDatabases(data)
                });
            }

            // AWS Services Tab
            const awsCount = counts.s3 + counts.memorydb + counts.secrets_manager + counts.bedrock;
            if (awsCount > 0) {
                tabs.push({ id: 'aws', label: '☁️ AWS Services', count: awsCount });
                tabContents.push({
                    id: 'aws',
                    content: renderAwsServices(data)
                });
            }

            if (tabs.length === 0) {
                document.getElementById('content').innerHTML = `
                    <div class="tabs-container">
                        <div class="empty-state">
                            <div class="empty-state-icon">📭</div>
                            <div>No configuration data available</div>
                        </div>
                    </div>
                `;
                return;
            }

            // Build tabs HTML
            const tabsHtml = tabs.map((tab, index) => `
                <button class="tab ${index === 0 ? 'active' : ''}" onclick="switchTab('${tab.id}')">
                    ${tab.label}
                    <span class="tab-badge">${tab.count}</span>
                </button>
            `).join('');

            const tabContentsHtml = tabContents.map((tab, index) => `
                <div id="tab-${tab.id}" class="tab-content ${index === 0 ? 'active' : ''}">
                    ${tab.content}
                </div>
            `).join('');

            document.getElementById('content').innerHTML = `
                <div class="tabs-container">
                    <div class="tabs">
                        ${tabsHtml}
                    </div>
                    ${tabContentsHtml}
                </div>
            `;
        }

        function switchTab(tabId) {
            // Remove active class from all tabs and contents
            document.querySelectorAll('.tab').forEach(tab => tab.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));

            // Add active class to selected tab and content
            event.target.closest('.tab').classList.add('active');
            document.getElementById(`tab-${tabId}`).classList.add('active');
        }

        function renderEnvironments(environments) {
            const sorted = Object.entries(environments).sort((a, b) => a[0].localeCompare(b[0]));

            return `
                <h2 class="section-header">Environment Variables</h2>
                <div>
                    ${sorted.map(([key, value]) => `
                        <div class="env-var">
                            <span class="env-key">${escapeHtml(key)}</span>
                            <span class="env-value">${escapeHtml(value)}</span>
                        </div>
                    `).join('')}
                </div>
            `;
        }

        function renderHttpApis(http) {
            const stats = calculateStats(http);

            return `
                <h2 class="section-header">HTTP API Endpoints</h2>
                <div class="stats-summary">
                    <div class="stat-card">
                        <div class="stat-number">${stats.total}</div>
                        <div class="stat-label">Total APIs</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">${stats.success}</div>
                        <div class="stat-label">Healthy</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">${stats.error}</div>
                        <div class="stat-label">Errors</div>
                    </div>
                </div>
                <div class="grid">
                    ${Object.entries(http).map(([key, result]) => `
                        <div class="check-item http">
                            <div class="check-header">
                                <span class="check-title">${escapeHtml(key)}</span>
                                <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                                    ${result.success ? '✓ Success' : '✗ Failed'}
                                </span>
                            </div>
                            <div class="check-details">
                                <div class="detail-row">
                                    <span class="detail-label">URL:</span>
                                    <span class="detail-value">${escapeHtml(result.url)}</span>
                                </div>
                                <div class="detail-row">
                                    <span class="detail-label">Method:</span>
                                    <span class="detail-value">${escapeHtml(result.method)}</span>
                                </div>
                                ${result.status_code ? `
                                    <div class="detail-row">
                                        <span class="detail-label">Status Code:</span>
                                        <span class="detail-value">${result.status_code}</span>
                                    </div>
                                ` : ''}
                                ${result.error ? `
                                    <div class="detail-row">
                                        <span class="detail-label">Error:</span>
                                        <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                                    </div>
                                ` : ''}
                            </div>
                        </div>
                    `).join('')}
                </div>
            `;
        }

        function renderDatabases(data) {
            let html = '<h2 class="section-header">Database Connections</h2>';

            // SQL Databases
            if (data.sql && Object.keys(data.sql).length > 0) {
                const stats = calculateStats(data.sql);
                html += `
                    <h3 style="color: #3b82f6; margin: 25px 0 15px; font-size: 1.2rem;">💾 SQL Databases</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Connected</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.sql).map(([key, result]) => renderSqlDatabase(key, result)).join('')}
                    </div>
                `;
            }

            // NoSQL Databases
            if (data.nosql && Object.keys(data.nosql).length > 0) {
                const stats = calculateStats(data.nosql);
                html += `
                    <h3 style="color: #10b981; margin: 25px 0 15px; font-size: 1.2rem;">🔥 NoSQL Databases</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #10b981 0%, #059669 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #10b981 0%, #059669 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Connected</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #10b981 0%, #059669 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.nosql).map(([key, result]) => renderNoSqlDatabase(key, result)).join('')}
                    </div>
                `;
            }

            // DynamoDB
            if (data.dynamodb && Object.keys(data.dynamodb).length > 0) {
                const stats = calculateStats(data.dynamodb);
                html += `
                    <h3 style="color: #06b6d4; margin: 25px 0 15px; font-size: 1.2rem;">⚡ DynamoDB Tables</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Accessible</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.dynamodb).map(([key, result]) => renderDynamoDB(key, result)).join('')}
                    </div>
                `;
            }

            return html;
        }

        function renderAwsServices(data) {
            let html = '<h2 class="section-header">AWS Cloud Services</h2>';

            // S3 Buckets
            if (data.s3 && Object.keys(data.s3).length > 0) {
                const stats = calculateStats(data.s3);
                html += `
                    <h3 style="color: #ec4899; margin: 25px 0 15px; font-size: 1.2rem;">🪣 S3 Buckets</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #ec4899 0%, #db2777 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #ec4899 0%, #db2777 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Accessible</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #ec4899 0%, #db2777 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.s3).map(([key, result]) => renderS3Bucket(key, result)).join('')}
                    </div>
                `;
            }

            // MemoryDB
            if (data.memorydb && Object.keys(data.memorydb).length > 0) {
                const stats = calculateStats(data.memorydb);
                html += `
                    <h3 style="color: #8b5cf6; margin: 25px 0 15px; font-size: 1.2rem;">💜 MemoryDB Clusters</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Accessible</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.memorydb).map(([key, result]) => renderMemoryDB(key, result)).join('')}
                    </div>
                `;
            }

            // Secrets Manager
            if (data.secrets_manager && Object.keys(data.secrets_manager).length > 0) {
                const stats = calculateStats(data.secrets_manager);
                html += `
                    <h3 style="color: #ef4444; margin: 25px 0 15px; font-size: 1.2rem;">🔐 Secrets Manager</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Accessible</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.secrets_manager).map(([key, result]) => renderSecretsManager(key, result)).join('')}
                    </div>
                `;
            }

            // Bedrock
            if (data.bedrock && Object.keys(data.bedrock).length > 0) {
                const stats = calculateStats(data.bedrock);
                html += `
                    <h3 style="color: #f97316; margin: 25px 0 15px; font-size: 1.2rem;">🤖 AWS Bedrock</h3>
                    <div class="stats-summary">
                        <div class="stat-card" style="background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);">
                            <div class="stat-number">${stats.total}</div>
                            <div class="stat-label">Total</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);">
                            <div class="stat-number">${stats.success}</div>
                            <div class="stat-label">Accessible</div>
                        </div>
                        <div class="stat-card" style="background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);">
                            <div class="stat-number">${stats.error}</div>
                            <div class="stat-label">Errors</div>
                        </div>
                    </div>
                    <div class="grid">
                        ${Object.entries(data.bedrock).map(([key, result]) => renderBedrock(key, result)).join('')}
                    </div>
                `;
            }

            return html;
        }

        function renderSqlDatabase(key, result) {
            return `
                <div class="check-item sql">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Connected' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Driver:</span>
                            <span class="detail-value">${escapeHtml(result.driver)}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Host:</span>
                            <span class="detail-value">${escapeHtml(result.host)}:${result.port}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Database:</span>
                            <span class="detail-value">${escapeHtml(result.database)}</span>
                        </div>
                        ${result.tables ? `
                            <div class="detail-row">
                                <span class="detail-label">Tables (${result.tables.length}):</span>
                            </div>
                            <ul class="list-items">
                                ${result.tables.slice(0, 10).map(t => `<li>${escapeHtml(t)}</li>`).join('')}
                                ${result.tables.length > 10 ? `<li>... and ${result.tables.length - 10} more</li>` : ''}
                            </ul>
                        ` : ''}
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function renderNoSqlDatabase(key, result) {
            return `
                <div class="check-item nosql">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Connected' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Driver:</span>
                            <span class="detail-value">${escapeHtml(result.driver)}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Host:</span>
                            <span class="detail-value">${escapeHtml(result.host)}:${result.port}</span>
                        </div>
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function renderS3Bucket(key, result) {
            return `
                <div class="check-item s3">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Accessible' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Bucket:</span>
                            <span class="detail-value">${escapeHtml(result.bucket)}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Region:</span>
                            <span class="detail-value">${escapeHtml(result.region)}</span>
                        </div>
                        ${result.object_count !== undefined ? `
                            <div class="detail-row">
                                <span class="detail-label">Objects:</span>
                                <span class="detail-value">${result.object_count}</span>
                            </div>
                        ` : ''}
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function renderMemoryDB(key, result) {
            return `
                <div class="check-item memorydb">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Accessible' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Cluster:</span>
                            <span class="detail-value">${escapeHtml(result.cluster)}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Region:</span>
                            <span class="detail-value">${escapeHtml(result.region)}</span>
                        </div>
                        ${result.status ? `
                            <div class="detail-row">
                                <span class="detail-label">Status:</span>
                                <span class="detail-value">${escapeHtml(result.status)}</span>
                            </div>
                        ` : ''}
                        ${result.node_count !== undefined ? `
                            <div class="detail-row">
                                <span class="detail-label">Nodes:</span>
                                <span class="detail-value">${result.node_count}</span>
                            </div>
                        ` : ''}
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function renderSecretsManager(key, result) {
            return `
                <div class="check-item secrets">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Accessible' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Secret:</span>
                            <span class="detail-value">${escapeHtml(result.secret_name)}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Region:</span>
                            <span class="detail-value">${escapeHtml(result.region)}</span>
                        </div>
                        ${result.version_id ? `
                            <div class="detail-row">
                                <span class="detail-label">Version:</span>
                                <span class="detail-value">${escapeHtml(result.version_id)}</span>
                            </div>
                        ` : ''}
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function renderDynamoDB(key, result) {
            return `
                <div class="check-item dynamodb">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Accessible' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Table:</span>
                            <span class="detail-value">${escapeHtml(result.table)}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Region:</span>
                            <span class="detail-value">${escapeHtml(result.region)}</span>
                        </div>
                        ${result.status ? `
                            <div class="detail-row">
                                <span class="detail-label">Status:</span>
                                <span class="detail-value">${escapeHtml(result.status)}</span>
                            </div>
                        ` : ''}
                        ${result.item_count !== undefined ? `
                            <div class="detail-row">
                                <span class="detail-label">Items:</span>
                                <span class="detail-value">${result.item_count.toLocaleString()}</span>
                            </div>
                        ` : ''}
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function renderBedrock(key, result) {
            return `
                <div class="check-item bedrock">
                    <div class="check-header">
                        <span class="check-title">${escapeHtml(key)}</span>
                        <span class="status-badge ${result.success ? 'status-success' : 'status-error'}">
                            ${result.success ? '✓ Accessible' : '✗ Failed'}
                        </span>
                    </div>
                    <div class="check-details">
                        <div class="detail-row">
                            <span class="detail-label">Region:</span>
                            <span class="detail-value">${escapeHtml(result.region)}</span>
                        </div>
                        ${result.model_count !== undefined ? `
                            <div class="detail-row">
                                <span class="detail-label">Models Available:</span>
                                <span class="detail-value">${result.model_count}</span>
                            </div>
                        ` : ''}
                        ${result.models && result.models.length > 0 ? `
                            <div class="detail-row">
                                <span class="detail-label">Top Models:</span>
                            </div>
                            <ul class="list-items">
                                ${result.models.slice(0, 5).map(m => `<li>${escapeHtml(m)}</li>`).join('')}
                                ${result.models.length > 5 ? `<li>... and ${result.models.length - 5} more</li>` : ''}
                            </ul>
                        ` : ''}
                        ${result.error ? `
                            <div class="detail-row">
                                <span class="detail-label">Error:</span>
                                <span class="detail-value" style="color: #ef476f;">${escapeHtml(result.error)}</span>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `;
        }

        function calculateStats(data) {
            const entries = Object.values(data);
            return {
                total: entries.length,
                success: entries.filter(e => e.success).length,
                error: entries.filter(e => !e.success).length
            };
        }

        function escapeHtml(text) {
            const map = {
                '&': '&amp;',
                '<': '&lt;',
                '>': '&gt;',
                '"': '&quot;',
                "'": '&#039;'
            };
            return String(text).replace(/[&<>"']/g, m => map[m]);
        }

        // Load data on page load
        loadData();

        // Refresh every 30 seconds
        setInterval(loadData, 30000);
    </script>
</body>
</html>
    "#;

    (StatusCode::OK, Html(html)).into_response()
}
