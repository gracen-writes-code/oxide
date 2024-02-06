FROM scratch
ADD fs.tar.gz /
CMD ["/sbin/init"]