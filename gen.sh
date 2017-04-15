git clone https://github.com/googleapis/googleapis.git
git clone https://github.com/google/protobuf.git

protoc -I ./protobuf/src -I ./googleapis/ --rust_out=src/ ./googleapis/google/datastore/v1beta3/*.proto  ./googleapis/google/type/latlng.proto ./protobuf/src/google/protobuf/wrappers.proto ./protobuf/src/google/protobuf/timestamp.proto ./protobuf/src/google/protobuf/struct.proto
protoc -I ./protobuf/src -I ./googleapis/ --rust-grpc_out=src/ ./googleapis/google/datastore/v1beta3/*.proto  ./googleapis/google/type/latlng.proto ./protobuf/src/google/protobuf/wrappers.proto ./protobuf/src/google/protobuf/timestamp.proto ./protobuf/src/google/protobuf/struct.proto
