# toc

- [models](./docs/models.md)

Ideas:

- Mi lenne ha saját háttérben futó appot tennénk a kliens gépekre, amik SMB hálózati mappában ellenőriznék,  
a saját md kartonjainkat. Minden fontos infót md kartonra tennénk, így egy mappában lenne egy md karton,  
ami tartalmazná a szükséges fileokat, belinkelve, a feladatokat és a változás logokat. Kérdéses, hogy egy online  
felület a hatékonyabb, vagy smb szinten a konkrét file, mappa munka. Bár itt kérdéses a jogosultság, ha nincs pl.:  
céges domain. Vagy az elnevezés konvenciók eltérései, Windows, Unix, Linux.

# 0.1.0

Goal

Store user files - currently just PDFs, and related structured data.  
Make it transparent and searchable.

File sturcture

1. Create a user data folder under ~/.bermuda
2. Store each PDF file here - under a new name
3. Store one information card under a yaml file.

