FROM postgres:latest
LABEL authors="wan"

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=test


# Copy the database schema to the /docker-entrypoint-initdb.d directory
COPY db.sql /docker-entrypoint-initdb.d/