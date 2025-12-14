Build a tool that parses a "log file" (a large raw string), and find line with specific keyword and return a list of matching keywords with proper structuring.

INPUT:
2025-12-14 [INFO] System boot initialized
2025-12-14 [WARN] Memory usage at 85%/n 
2025-12-14 [ERROR] Database connection failed: Timeout
2025-12-14 [INFO] Retrying connection...
2025-12-14 [ERROR] Critical failure: Disk full

while i<n 
    let start,end;
    start = 0
    Tag - INFO,WARN,ERROR
    0 - TAG - DATE


OUTPUT: A vector of LogEntry
Found Error:
  Date:    "2025-12-14"
  Message: "Database connection failed: Timeout"

Found Error:
  Date:    "2025-12-14"
  Message: "Critical failure: Disk full"
