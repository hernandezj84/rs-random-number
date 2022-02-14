FROM debian:11
RUN mkdir /binaries
COPY target /binaries/target
CMD ["/bin/bash", "-c"]