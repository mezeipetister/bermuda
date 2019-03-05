# 0.1.0

Goal

Store user files - currently just PDFs, and related structured data.  
Make it transparent and searchable.

File sturcture

1. Create a user data folder under ~/.bermuda
2. Store each PDF file here - under a new name
3. Store one information card under a yaml file.

Both 2. and 3. use the same file name but different file extensions:  
- ID.pdf and,
- ID.yaml

ID is going to be the file archive timestamp.

Information card yaml

Document name: String,
Document description: String,
Received time: Date,
Archived time: Date,
Due date: Date,
Folder: Int
