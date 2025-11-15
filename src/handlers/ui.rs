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
            margin-bottom: 40px;
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

        .card {
            background: white;
            border-radius: 12px;
            padding: 25px;
            margin-bottom: 25px;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
            transition: transform 0.2s, box-shadow 0.2s;
        }

        .card:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
        }

        .card h2 {
            color: #667eea;
            margin-bottom: 20px;
            font-size: 1.5rem;
            border-bottom: 2px solid #f0f0f0;
            padding-bottom: 10px;
        }

        .card h3 {
            color: #764ba2;
            margin: 20px 0 10px;
            font-size: 1.2rem;
        }

        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
        }

        .status-badge {
            display: inline-block;
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 0.85rem;
            font-weight: 600;
            margin-left: 10px;
        }

        .status-success {
            background: #d4edda;
            color: #155724;
        }

        .status-error {
            background: #f8d7da;
            color: #721c24;
        }

        .env-var {
            display: flex;
            justify-content: space-between;
            padding: 10px 15px;
            background: #f8f9fa;
            border-radius: 6px;
            margin-bottom: 8px;
            font-family: 'Courier New', monospace;
            font-size: 0.9rem;
        }

        .env-var:nth-child(even) {
            background: #e9ecef;
        }

        .env-key {
            font-weight: 600;
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
            background: #f8f9fa;
            border-radius: 8px;
            padding: 15px;
            margin-bottom: 15px;
        }

        .check-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
        }

        .check-title {
            font-weight: 600;
            color: #495057;
            font-size: 1rem;
        }

        .check-details {
            display: grid;
            gap: 8px;
            font-size: 0.9rem;
            color: #6c757d;
        }

        .detail-row {
            display: flex;
            justify-content: space-between;
            padding: 6px 0;
            border-bottom: 1px solid #e9ecef;
        }

        .detail-row:last-child {
            border-bottom: none;
        }

        .detail-label {
            font-weight: 500;
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
            padding: 40px 20px;
            color: #6c757d;
            font-style: italic;
        }

        .list-items {
            margin-top: 10px;
            padding-left: 20px;
        }

        .list-items li {
            padding: 4px 0;
            color: #6c757d;
        }

        @media (max-width: 768px) {
            header h1 {
                font-size: 1.8rem;
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
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>PMP Test API</h1>
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
        async function loadData() {
            try {
                const response = await fetch('/_/info');

                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                const data = await response.json();
                renderData(data);
            } catch (error) {
                document.getElementById('content').innerHTML = `
                    <div class="error">
                        <h2>Error Loading Data</h2>
                        <p>${error.message}</p>
                    </div>
                `;
            }
        }

        function renderData(data) {
            let html = '';

            // Environment Variables
            if (data.environments && Object.keys(data.environments).length > 0) {
                html += `
                    <div class="card">
                        <h2>Environment Variables (${Object.keys(data.environments).length})</h2>
                        <div>
                            ${Object.entries(data.environments)
                                .sort((a, b) => a[0].localeCompare(b[0]))
                                .map(([key, value]) => `
                                    <div class="env-var">
                                        <span class="env-key">${escapeHtml(key)}</span>
                                        <span class="env-value">${escapeHtml(value)}</span>
                                    </div>
                                `).join('')}
                        </div>
                    </div>
                `;
            }

            // SQL Databases
            if (data.sql && Object.keys(data.sql).length > 0) {
                html += renderCheckSection('SQL Databases', data.sql, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // NoSQL Databases
            if (data.nosql && Object.keys(data.nosql).length > 0) {
                html += renderCheckSection('NoSQL Databases', data.nosql, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // HTTP APIs
            if (data.http && Object.keys(data.http).length > 0) {
                html += renderCheckSection('HTTP APIs', data.http, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // S3 Buckets
            if (data.s3 && Object.keys(data.s3).length > 0) {
                html += renderCheckSection('S3 Buckets', data.s3, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // MemoryDB
            if (data.memorydb && Object.keys(data.memorydb).length > 0) {
                html += renderCheckSection('MemoryDB Clusters', data.memorydb, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // Secrets Manager
            if (data.secrets_manager && Object.keys(data.secrets_manager).length > 0) {
                html += renderCheckSection('AWS Secrets Manager', data.secrets_manager, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // DynamoDB
            if (data.dynamodb && Object.keys(data.dynamodb).length > 0) {
                html += renderCheckSection('DynamoDB Tables', data.dynamodb, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            // Bedrock
            if (data.bedrock && Object.keys(data.bedrock).length > 0) {
                html += renderCheckSection('AWS Bedrock', data.bedrock, (key, result) => `
                    <div class="check-item">
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
                                    <span class="detail-value" style="color: #c33;">${escapeHtml(result.error)}</span>
                                </div>
                            ` : ''}
                        </div>
                    </div>
                `);
            }

            if (!html) {
                html = '<div class="empty-state">No configuration data available</div>';
            }

            document.getElementById('content').innerHTML = html;
        }

        function renderCheckSection(title, data, itemRenderer) {
            return `
                <div class="card">
                    <h2>${title} (${Object.keys(data).length})</h2>
                    ${Object.entries(data).map(([key, value]) => itemRenderer(key, value)).join('')}
                </div>
            `;
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
