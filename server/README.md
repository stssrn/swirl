# Swirl API routes

## GET repos
Get all public repositories.

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
Get list of repository commits.

### Parameters
| Name    | Type | Description                       | Default     |
|---------|------|-----------------------------------|-------------|
| :repo   | str  | repo field in _config.yaml_       |             |
| &branch | str  | branch to list commits from       | HEAD        |
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
Get info of commit.

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
Get list of branches.

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
Get file tree.

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
Get content of file.

### Parameters
| Name     | Type  | Description                 | Default |
|----------|-------|-----------------------------|---------|
| :repo    | str   | Repo field in _config.yaml_ |         |
| :path    | str   | File path                   |         |
| &branch  | str   | Name of branch              | HEAD    |

### Response
```
[File content]
```

---

## GET repos/:repo/raw/is_bin/:path
Get "true" or "false depending on wether requested file is binary or not.

### Parameters
| Name     | Type  | Description                 | Default |
|----------|-------|-----------------------------|---------|
| :repo    | str   | Repo field in _config.yaml_ |         |
| :path    | str   | File path                   |         |
| &branch  | str   | Name of branch              | HEAD    |

### Response
```
{ bool }
```

## GET repos/:repo/raw/readme
Get readme content.

### Parameters
| Name     | Type  | Description                 | Default |
|----------|-------|-----------------------------|---------|
| :repo    | str   | Repo field in _config.yaml_ |         |

### Response
```
[File content]
```
