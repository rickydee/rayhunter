need to run this every boot
`docker run --privileged --rm tonistiigi/binfmt --install all`
`docker buildx build --platform linux/arm/v7 .`
