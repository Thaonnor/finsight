import sqlite3
from datetime import datetime, timedelta
import random

conn = sqlite3.connect('src-tauri/finsight.db')
cursor = conn.cursor()

# Categories (Uncategorized already seeded by system)
cursor.execute("INSERT INTO categories (name, parent_id) VALUES ('Food', NULL)")
cursor.execute("INSERT INTO categories (name, parent_id) VALUES ('Groceries', 2)")  # Child of Food
cursor.execute("INSERT INTO categories (name, parent_id) VALUES ('Transportation', NULL)")

# 4 Mock Accounts
accounts = [
    ("Chase Checking", "checking"),
    ("Wells Fargo Savings", "savings"),
    ("Credit Union Checking", "checking"),
    ("High Yield Savings", "savings")
]

for name, acc_type in accounts:
    cursor.execute("INSERT INTO accounts (name, account_type) VALUES (?, ?)", (name, acc_type))

# Get account IDs for transactions
cursor.execute("SELECT id FROM accounts")
account_ids = [row[0] for row in cursor.fetchall()]

# Transaction templates
transactions = [
    ("Whole Foods", "debit", 3),  # Groceries category
    ("Starbucks", "debit", 2),   # Food category
    ("Gas Station", "debit", 4), # Transportation
    ("Salary Deposit", "credit", 1), # Uncategorized
    ("Restaurant", "debit", 2),   # Food
    ("Uber", "debit", 4),        # Transportation
    ("Grocery Store", "debit", 3), # Groceries
    ("Interest Payment", "credit", 1), # Uncategorized
]

# Generate transactions for each account
base_date = datetime.now() - timedelta(days=30)
for account_id in account_ids:
    num_transactions = random.randint(5, 10)
    for i in range(num_transactions):
        desc, tx_type, category_id = random.choice(transactions)
        amount = random.randint(500, 15000)  # $5.00 to $150.00 in cents
        date = (base_date + timedelta(days=random.randint(0, 30))).strftime('%Y-%m-%d')
        
        cursor.execute("""
            INSERT INTO transactions (account_id, amount_cents, transaction_type, description, transaction_date, category_id)
            VALUES (?, ?, ?, ?, ?, ?)
        """, (account_id, amount, tx_type, desc, date, category_id))

conn.commit()
conn.close()
print("Test data seeded successfully.")