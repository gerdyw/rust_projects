#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${1:-http://localhost:8000}"
TEST_IMAGES_DIR="${2:-./test_images}"

if [[ ! -d "$TEST_IMAGES_DIR" ]]; then
  echo "Error: test images directory not found: $TEST_IMAGES_DIR" >&2
  exit 1
fi

mapfile -t files < <(find "$TEST_IMAGES_DIR" -maxdepth 1 -type f | sort)

if (( ${#files[@]} < 3 )); then
  echo "Error: expected at least 3 images in $TEST_IMAGES_DIR, found ${#files[@]}" >&2
  exit 1
fi

echo "Creating job at $BASE_URL/api/create ..."
create_response="$(curl -sS -X POST "$BASE_URL/api/create" \
  -H "Content-Type: application/json" \
  -d '{"image_count":3}')"

job_id="$(printf '%s' "$create_response" | sed -n 's/.*"job_id"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')"

if [[ -z "$job_id" ]]; then
  echo "Error: failed to parse job_id from response: $create_response" >&2
  exit 1
fi

echo "Created job_id: $job_id"

for idx in 0 1 2; do
  file="${files[$idx]}"

  mime_type="application/octet-stream"
  if command -v file >/dev/null 2>&1; then
    mime_type="$(file --brief --mime-type "$file")"
  fi

  echo "Uploading idx=$idx file=$file (mime=$mime_type)"
  status="$(curl -sS -o /tmp/submit_image_${idx}.out -w "%{http_code}" \
    -X POST "$BASE_URL/api/submit-image/$job_id/$idx" \
    -H "Content-Type: $mime_type" \
    --data-binary "@$file")"

  if [[ "$status" != "202" ]]; then
    echo "Error: upload failed for idx=$idx with HTTP $status" >&2
    cat /tmp/submit_image_${idx}.out >&2 || true
    exit 1
  fi

done

echo "All 3 uploads submitted successfully for job_id=$job_id"
echo "Check status: curl -sS $BASE_URL/api/$job_id"
