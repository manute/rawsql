-- name: simple
-- command: query
SELECT * FROM table1 u where  u.name = ?;

-- name: two-lines
-- command: exec
Insert INTO table2
SELECT * FROM table1;

-- name: complex
-- command: query
SELECT *
FROM Customers c
INNER JOIN CustomerAccounts ca
    ON ca.CustomerID = c.CustomerID
    AND c.State = ?
INNER JOIN Accounts a
    ON ca.AccountID = a.AccountID
    AND a.Status = ?;
