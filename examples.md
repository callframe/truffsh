## Invoking Makefile

```bash
MAKEFILE_LOCATION=$1
if [ -z "$1" ]; then
    echo "Usage: $0 <makefile_location>"
    exit 1
fi

make -f "$1"
```

```neosh
argv :: arguments
if len(argv) < 2 {
    echo "No arguments provided"
    exit 1
}

makefile_location :: argv[1]
make -f "$makefile_location"
```


