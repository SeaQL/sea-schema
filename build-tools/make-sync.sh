rm -rf sea-schema-sync/src
rm -rf sea-schema-sync/tests
cp -r src sea-schema-sync
mkdir -p sea-schema-sync/tests/writer
mkdir -p sea-schema-sync/tests/live
mkdir -p sea-schema-sync/tests/discovery
cp -r tests/writer/sqlite sea-schema-sync/tests/writer
cp -r tests/live/sqlite sea-schema-sync/tests/live
cp -r tests/discovery/sqlite sea-schema-sync/tests/discovery
cp -r tests/sakila sea-schema-sync/tests
rm -rf sea-schema-sync/src/bin
cd sea-schema-sync
find src   -type f -name '*.rs' -exec sed -i '' 's/async //' {} +
find tests -type f -name '*.rs' -exec sed -i '' 's/async //' {} +
find src   -type f -name '*.rs' -exec sed -i '' 's/\.await//' {} +
find tests -type f -name '*.rs' -exec sed -i '' 's/\.await//' {} +
find src -type f -name '*.rs' -exec sed -i '' 's/not(feature = "sqlx-sqlite")/all(not(feature = "rusqlite"), not(feature = "sqlx-sqlite"))/' {} +
find src   -type f -name '*.rs' -exec sed -i '' 's/#\[cfg(feature = "sqlx-sqlite")\]/#\[cfg(feature = "rusqlite")\]/' {} +
find tests -type f -name 'Cargo.toml' -exec sed -i '' 's/sea-schema/sea-schema-sync/' {} +
find tests -type f -name 'Cargo.toml' -exec sed -i '' 's/sqlx-sqlite/rusqlite/' {} +
find tests -type f -name 'Cargo.toml' -exec sed -i '' 's/, "runtime-async-std-native-tls"//' {} +
find tests -type f -name 'Cargo.toml' -exec sed -i '' '/sqlx/d' {} +
find tests -type f -name 'Cargo.toml' -exec sed -i '' '/async-std/d' {} +
find tests -type f -name '*.rs' -exec sed -i '' '/async_std/d' {} +
find tests -type f -name 'Cargo.toml' -exec sed -i '' 's/, default-features = false//' {} +
find src   -type f -name '*.rs' -exec sed -i '' 's/sqlx_types::/rusqlite_types::/' {} +
find tests -type f -name '*.rs' -exec sed -i '' 's/sqlx_types::/rusqlite_types::/' {} +
find src -type f -name '*.rs' -exec sed -i '' "s/SqlxRow/RusqliteRow/g" {} +
find src -type f -name '*.rs' -exec sed -i '' 's/SqlxError/RusqliteError/g' {} +
find src -type f -name '*.rs' -exec sed -i '' 's/SQLx/Rusqlite/' {} +
find src -type f -name '*.rs' -exec sed -i '' 's/SqlitePool/RusqliteConnection/' {} +
find src -type f -name '*.rs' -exec sed -i '' '/#\[async_trait::async_trait/d' {} +
find src -type f -name '*.rs' -exec sed -i '' 's/ \+ Sync//' {} +
cargo fmt
