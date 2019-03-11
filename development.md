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

## Backlog

| Description                                                       | Planning | InProgress | Blocking | Review | Done |
|-------------------------------------------------------------------|----------|------------|----------|--------|------|
| Demo task, lets see what happens.                                 | x        |            |          |        |      |
| Init process[^3]                                                  |          | x          |          |        |      |
| Create ~/.bermuda folder if does not exist as initialisation[^2]. |          | x          |          |        |      |
| File read library by (path, extension) -> File path list[^4]      | x        |            |          |        |      |
| File write library (content, path(by extension))[^5]              | x        |            |          |        |      |
| CLI basics[^6]                                                    | x        |            |          |        |      |
|                                                                   |          |            |          |        |      |


[^2]: Initial process

[^3]: Run all the needed init stuff, call it once at start

[^4]: Function that can create a "map" by a folder. Like what files with this specifig parameters (..) we have? Give me my list.

[^5]: Helper function to write content to a file.

[^6]: Basics of a CLI application. Consider to split it into bin packages for smaller packages as separated CLI apps, and a main CLI app.
