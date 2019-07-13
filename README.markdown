# postgrust
[WIP] PostgreSQL Debug CLI


```
A PostgreSQL performance debug CLI <beta>

USAGE:
postgrust [OPTIONS]

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-q, --query <query>     Query to be executed [possible values: SeqScans, ActiveQueries]
-t, --test <pg_test>    Tests if connection to PostgreSQL database(s) can be established [default: true]
```


To run with your config,

Copy paste `PGConfig.toml` to `/usr/local/postgrust` with the same name and make necessary changes. 

#### Supported Operations

- [x] active queries `-q ActiveQueries`
- [x] sequential scan `-q SeqScans`
