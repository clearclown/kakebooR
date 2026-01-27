// kakebooR Frontend Application

const API_BASE = '/api';

// ============================================================================
// API Client
// ============================================================================

const api = {
    async get(endpoint) {
        const response = await fetch(`${API_BASE}${endpoint}`);
        if (!response.ok) {
            throw new Error(`API error: ${response.status}`);
        }
        return response.json();
    },

    async post(endpoint, data) {
        const response = await fetch(`${API_BASE}${endpoint}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data),
        });
        if (!response.ok) {
            throw new Error(`API error: ${response.status}`);
        }
        return response.json();
    },

    async delete(endpoint) {
        const response = await fetch(`${API_BASE}${endpoint}`, {
            method: 'DELETE',
        });
        if (!response.ok) {
            throw new Error(`API error: ${response.status}`);
        }
        return response.ok;
    },
};

// ============================================================================
// Utility Functions
// ============================================================================

function formatCurrency(amount) {
    return new Intl.NumberFormat('ja-JP', {
        style: 'currency',
        currency: 'JPY',
    }).format(amount);
}

function formatDate(dateStr) {
    const date = new Date(dateStr);
    return date.toLocaleDateString('ja-JP');
}

function clearApp() {
    const app = document.getElementById('app');
    while (app.firstChild) {
        app.removeChild(app.firstChild);
    }
    return app;
}

function showLoading() {
    const app = clearApp();
    const container = document.createElement('div');
    container.className = 'text-center py-8';

    const spinner = document.createElement('div');
    spinner.className = 'inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500';
    container.appendChild(spinner);

    const text = document.createElement('p');
    text.className = 'mt-2 text-gray-600';
    text.textContent = '読み込み中...';
    container.appendChild(text);

    app.appendChild(container);
}

function showError(message) {
    const app = clearApp();
    const template = document.getElementById('error-template');
    const clone = template.content.cloneNode(true);
    clone.getElementById('error-message').textContent = message;
    app.appendChild(clone);
}

// ============================================================================
// DOM Helper Functions
// ============================================================================

function createElement(tag, className, textContent) {
    const el = document.createElement(tag);
    if (className) el.className = className;
    if (textContent) el.textContent = textContent;
    return el;
}

function createLink(href, className, textContent) {
    const link = document.createElement('a');
    link.href = href;
    if (className) link.className = className;
    if (textContent) link.textContent = textContent;
    return link;
}

function createButton(className, textContent, onClick) {
    const btn = document.createElement('button');
    if (className) btn.className = className;
    if (textContent) btn.textContent = textContent;
    if (onClick) btn.addEventListener('click', onClick);
    return btn;
}

function createInput(type, id, name, className, attrs) {
    const input = document.createElement('input');
    input.type = type;
    input.id = id;
    input.name = name;
    if (className) input.className = className;
    if (attrs) {
        Object.entries(attrs).forEach(([key, value]) => {
            input.setAttribute(key, value);
        });
    }
    return input;
}

function createSelect(id, name, className, options) {
    const select = document.createElement('select');
    select.id = id;
    select.name = name;
    if (className) select.className = className;
    select.required = true;

    options.forEach(opt => {
        const option = document.createElement('option');
        option.value = opt.value;
        option.textContent = opt.label;
        select.appendChild(option);
    });

    return select;
}

function createFormGroup(labelText, inputElement) {
    const group = document.createElement('div');

    const label = document.createElement('label');
    label.className = 'block text-sm font-medium text-gray-700 mb-1';
    label.textContent = labelText;
    group.appendChild(label);

    group.appendChild(inputElement);
    return group;
}

// ============================================================================
// Dashboard View
// ============================================================================

async function showDashboard() {
    showLoading();

    try {
        const now = new Date();
        const year = now.getFullYear();
        const month = now.getMonth() + 1;

        const report = await api.get(`/reports/monthly/?year=${year}&month=${month}`);

        const app = clearApp();
        const template = document.getElementById('dashboard-template');
        const clone = template.content.cloneNode(true);

        // Set date display
        clone.getElementById('date-display').textContent = `${year}年${month}月`;

        // Set amounts
        clone.getElementById('income-amount').textContent = formatCurrency(report.total_income);
        clone.getElementById('expense-amount').textContent = formatCurrency(report.total_expense);
        clone.getElementById('balance-amount').textContent = formatCurrency(report.net_balance);

        // Set balance card color based on positive/negative
        const balanceCard = clone.getElementById('balance-card');
        const balanceLabel = clone.getElementById('balance-label');
        const balanceAmount = clone.getElementById('balance-amount');

        if (report.net_balance >= 0) {
            balanceCard.classList.add('bg-blue-100');
            balanceLabel.classList.add('text-blue-800');
            balanceAmount.classList.add('text-blue-600');
        } else {
            balanceCard.classList.add('bg-yellow-100');
            balanceLabel.classList.add('text-yellow-800');
            balanceAmount.classList.add('text-yellow-600');
        }

        app.appendChild(clone);
    } catch (error) {
        showError('ダッシュボードの読み込みに失敗しました: ' + error.message);
    }
}

// ============================================================================
// Transactions List View
// ============================================================================

async function showTransactions() {
    showLoading();

    try {
        const [transactionsRes, categoriesRes] = await Promise.all([
            api.get('/transactions/'),
            api.get('/categories/'),
        ]);

        // Extract results arrays from API responses
        const transactions = transactionsRes.results || [];
        const categories = categoriesRes.results || [];

        const categoryMap = new Map(categories.map(c => [c.id, c]));

        const app = clearApp();
        const container = document.createElement('div');

        // Header
        const header = document.createElement('div');
        header.className = 'flex justify-between items-center mb-6';

        const title = createElement('h2', 'text-2xl font-bold', '取引一覧');
        header.appendChild(title);

        const newBtn = createLink('#new', 'bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded', '新規取引');
        header.appendChild(newBtn);

        container.appendChild(header);

        if (transactions.length === 0) {
            const empty = createElement('p', 'text-gray-600 text-center py-8', '取引がありません。新規取引を追加してください。');
            container.appendChild(empty);
        } else {
            // Table
            const table = document.createElement('table');
            table.className = 'w-full bg-white rounded-lg shadow overflow-hidden';

            // Table header
            const thead = document.createElement('thead');
            thead.className = 'bg-gray-50';

            const headerRow = document.createElement('tr');
            const headerLabels = ['日付', 'カテゴリ', '説明', '金額', '操作'];
            const headerClasses = [
                'px-4 py-3 text-left text-sm font-semibold text-gray-600',
                'px-4 py-3 text-left text-sm font-semibold text-gray-600',
                'px-4 py-3 text-left text-sm font-semibold text-gray-600',
                'px-4 py-3 text-right text-sm font-semibold text-gray-600',
                'px-4 py-3 text-center text-sm font-semibold text-gray-600',
            ];

            headerLabels.forEach((label, i) => {
                const th = createElement('th', headerClasses[i], label);
                headerRow.appendChild(th);
            });

            thead.appendChild(headerRow);
            table.appendChild(thead);

            // Table body
            const tbody = document.createElement('tbody');
            tbody.className = 'divide-y divide-gray-200';

            // Sort by date descending
            transactions.sort((a, b) => new Date(b.transaction_date) - new Date(a.transaction_date));

            for (const tx of transactions) {
                const row = document.createElement('tr');
                row.className = 'hover:bg-gray-50';

                const category = categoryMap.get(tx.category_id);
                const isExpense = tx.transaction_type === 'expense';

                // Date cell
                const dateCell = createElement('td', 'px-4 py-3 text-sm', formatDate(tx.transaction_date));
                row.appendChild(dateCell);

                // Category cell
                const categoryCell = createElement('td', 'px-4 py-3 text-sm', category ? category.name : '不明');
                row.appendChild(categoryCell);

                // Description cell
                const descCell = createElement('td', 'px-4 py-3 text-sm', tx.description);
                row.appendChild(descCell);

                // Amount cell
                const amountClass = `px-4 py-3 text-sm text-right font-medium ${isExpense ? 'text-red-600' : 'text-green-600'}`;
                const amountCell = createElement('td', amountClass, (isExpense ? '-' : '+') + formatCurrency(tx.amount));
                row.appendChild(amountCell);

                // Actions cell
                const actionsCell = createElement('td', 'px-4 py-3 text-center');
                const deleteBtn = createButton('text-red-600 hover:text-red-800 text-sm', '削除', () => deleteTransaction(tx.id));
                actionsCell.appendChild(deleteBtn);
                row.appendChild(actionsCell);

                tbody.appendChild(row);
            }

            table.appendChild(tbody);
            container.appendChild(table);
        }

        app.appendChild(container);
    } catch (error) {
        showError('取引一覧の読み込みに失敗しました: ' + error.message);
    }
}

async function deleteTransaction(id) {
    if (!confirm('この取引を削除しますか？')) {
        return;
    }

    try {
        await api.delete(`/transactions/${id}/`);
        showTransactions();
    } catch (error) {
        alert('削除に失敗しました: ' + error.message);
    }
}

// ============================================================================
// New Transaction Form
// ============================================================================

async function showNewTransactionForm() {
    showLoading();

    try {
        const categoriesRes = await api.get('/categories/');
        const categories = categoriesRes.results || [];

        const app = clearApp();
        const container = createElement('div', 'max-w-md mx-auto');

        const title = createElement('h2', 'text-2xl font-bold mb-6 text-center', '新規取引');
        container.appendChild(title);

        const form = document.createElement('form');
        form.className = 'bg-white rounded-lg shadow p-6 space-y-4';
        form.addEventListener('submit', handleNewTransaction);

        const inputClass = 'w-full border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500';

        // Transaction type
        const typeSelect = createSelect('transaction_type', 'transaction_type', inputClass, [
            { value: 'expense', label: '支出' },
            { value: 'income', label: '収入' },
        ]);
        form.appendChild(createFormGroup('取引種別', typeSelect));

        // Category
        const categorySelect = createSelect('category_id', 'category_id', inputClass, []);
        form.appendChild(createFormGroup('カテゴリ', categorySelect));

        // Update category options when type changes
        function updateCategories() {
            const selectedType = typeSelect.value;
            while (categorySelect.firstChild) {
                categorySelect.removeChild(categorySelect.firstChild);
            }

            const filtered = categories.filter(c => c.category_type === selectedType);
            if (filtered.length === 0) {
                const option = document.createElement('option');
                option.value = '';
                option.textContent = 'カテゴリがありません';
                categorySelect.appendChild(option);
            } else {
                filtered.forEach(cat => {
                    const option = document.createElement('option');
                    option.value = cat.id;
                    option.textContent = cat.name;
                    categorySelect.appendChild(option);
                });
            }
        }

        typeSelect.addEventListener('change', updateCategories);
        updateCategories();

        // Amount
        const amountInput = createInput('number', 'amount', 'amount', inputClass, {
            min: '1',
            required: 'required',
            placeholder: '1000',
        });
        form.appendChild(createFormGroup('金額', amountInput));

        // Date
        const today = new Date().toISOString().split('T')[0];
        const dateInput = createInput('date', 'transaction_date', 'transaction_date', inputClass, {
            required: 'required',
            value: today,
        });
        form.appendChild(createFormGroup('日付', dateInput));

        // Description
        const descInput = createInput('text', 'description', 'description', inputClass, {
            required: 'required',
            placeholder: 'ランチ、電車賃など',
        });
        form.appendChild(createFormGroup('説明', descInput));

        // Submit button
        const submitGroup = createElement('div', 'pt-4');
        const submitBtn = createButton(
            'w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded',
            '登録'
        );
        submitBtn.type = 'submit';
        submitGroup.appendChild(submitBtn);
        form.appendChild(submitGroup);

        container.appendChild(form);

        // Back link
        const backP = createElement('p', 'text-center mt-4');
        const backLink = createLink('#transactions', 'text-blue-600 hover:underline', '取引一覧に戻る');
        backP.appendChild(backLink);
        container.appendChild(backP);

        app.appendChild(container);
    } catch (error) {
        showError('フォームの読み込みに失敗しました: ' + error.message);
    }
}

async function handleNewTransaction(event) {
    event.preventDefault();

    const form = event.target;
    const data = {
        amount: parseInt(form.amount.value, 10),
        category_id: parseInt(form.category_id.value, 10),
        description: form.description.value,
        transaction_date: form.transaction_date.value,
        transaction_type: form.transaction_type.value,
    };

    try {
        await api.post('/transactions/', data);
        window.location.hash = '#transactions';
    } catch (error) {
        alert('登録に失敗しました: ' + error.message);
    }
}

// ============================================================================
// Router
// ============================================================================

function router() {
    const hash = window.location.hash || '#dashboard';

    switch (hash) {
        case '#dashboard':
            showDashboard();
            break;
        case '#transactions':
            showTransactions();
            break;
        case '#new':
            showNewTransactionForm();
            break;
        default:
            showDashboard();
    }
}

// Listen for hash changes
window.addEventListener('hashchange', router);

// Initial route
document.addEventListener('DOMContentLoaded', router);
