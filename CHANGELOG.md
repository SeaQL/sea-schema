# Changelog

All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.2.6 - 2021-08-21

+ Use sea-query to 0.15
+ [[#13]] Added `is_identity` to Postgres `ColumnInfo`

[#13]: https://github.com/SeaQL/sea-schema/issues/13

## 0.2.5 - 2021-08-14

+ improve Postgres schema discovery

## 0.2.4 - 2021-08-07

+ improve Postgres schema discovery

## 0.2.3 - 2021-06-19

+ Improve `ColumnType` output of MySQL writer

## 0.2.2 - 2021-06-19

+ Added `ColumnExpression` to MySQL ColumnInfo output
+ Postgres type definitions

## 0.2.1 - 2021-04-30

+ Foreign key writer
+ Index prefix and order

## 0.2.0 - 2021-04-25

+ `Writer`
+ changed `StringAttr` definition
+ added `IndexPart` definition

## 0.1.4 - 2021-04-13

+ serde support on types
+ Query table's `char_set` from information_schema

## 0.1.3 - 2021-04-11

+ `TableInfo` includes `char_set`

## 0.1.2 - 2021-04-11

+ Restructure dependencies

## 0.1.1 - 2021-04-08

+ Fix docs.rs

## 0.1.0 - 2021-04-08

+ Initial release
