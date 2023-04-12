FROM centos:latest

RUN sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-Linux-* && \
    sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-Linux-* && \
    yum update -y && \
    yum -y install python3 \
    python3-pip \
    python3-devel \
    gcc \
    rpm-build \
    libffi-devel \
    openssl-devel

RUN pip3 install --upgrade pip && \
    pip3 install pywinrm && \
    pip3 install fastapi && \
    pip3 install uvicorn

