if ! cargo fmt --check; then
  echo "❌ Format check failed!"
  exit 1
fi
echo "✅ Format check succeeded!"

if ! cargo test; then
    echo "❌ Base tests failed!"
    exit 1
fi
echo "✅ Base tests succeeded!"

if ! cargo test --all-features; then
    echo "❌ Feature tests failed!"
    exit 1
fi
echo "✅ Feature tests succeeded!"