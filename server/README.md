# Swirl API routes

## GET repos
Get all (public) repositories on the server

### Parameters
| Name   | Type | Description                      | Default |
|--------|------|----------------------------------|---------|
| &page  | int  | Page number of results to return | 1       |
| &limit | int  | Page size of results             | All     |

### Resonse
```
[
    name: string,
    repo: string,
    note: string,
    readme: string?,
]
```

---

## GET repos/:repo/commits
Get repository commits

### Parameters
| Name    | Type | Description                       | Default     |
|---------|------|-----------------------------------|-------------|
| :repo   | str  | repo field in _config.yaml_       |             |
| &branch | str  | branch to list commits from       |             |
| &page   | int  | Page number of results to return  | 1           |
| &limit  | int  | Page size of results              | 20          |

### Response
```
[
    id: string,
    message: string,
]
```

---

## GET repos/:repo/commits/:commit
Get note from commit in repository

### Parameters
| Name      | Type | Description                 |
|-----------|------|-----------------------------|
| :repo     | str  | repo field in _config.yaml_ |
| :commit   | str  | commit SHA                  |

### Response
```
author: {
            name: string,
            email: string,
            pgp_key: string,
        },
timestamp: int,
message: string,
diff: string,
```

---

## GET repos/:repo/branches
### Parameters
| Name   | Type | Description                      | Default |
|--------|------|----------------------------------|---------|
| :repo  | str  | Repo field in _config.yaml_      |         |
| &page  | int  | Page number of results to return | 1       |
| &limit | int  | Page size of results             | All     |

### Response
```
[
    name: string,
]
```

---

## GET repos/:repo/branches/:branch/tree
Get tree of most recent commit in branch
### Parameters
| Name    | Type | Description                      | Default |
|---------|------|----------------------------------|---------|
| :repo   | str  | Repo field in _config.yaml_      |         |
| :branch | str  | SHA of branch                    |         |
| &page   | int  | Page number of results to return | 1       |
| &limit  | int  | Page size of results             | 50      |

### Response
```
[
    name: string,
    id: string
    entries: [ ... recurseive ]?,
]
```

---

## GET repos/:repo/raw/content/:path
Get content of file

### Parameters
| Name     | Type  | Description                 | Default |
|----------|-------|-----------------------------|---------|
| :repo    | str   | Repo field in _config.yaml_ |         |
| :path    | str   | File path                   |         |
| &branch  | str   | Name of branch              | main    |

### Response
```
[File content]
```

---

## GET repos/:repo/raw/is_bin/:path
Get content of file

### Parameters
| Name     | Type  | Description                 | Default |
|----------|-------|-----------------------------|---------|
| :repo    | str   | Repo field in _config.yaml_ |         |
| :path    | str   | File path                   |         |
| &branch  | str   | Name of branch              | main    |

### Response
```
{ bool }
```
