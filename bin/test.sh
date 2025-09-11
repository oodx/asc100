#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=================================="
echo "ASC100 Test Suite"
echo "=================================="
echo ""

# Run cargo tests
echo -e "${YELLOW}Running unit tests...${NC}"
if cargo test --lib; then
    echo -e "${GREEN}✓ Unit tests passed${NC}"
else
    echo -e "${RED}✗ Unit tests failed${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Running integration tests...${NC}"
if cargo run --quiet; then
    echo -e "${GREEN}✓ Integration tests passed${NC}"
else
    echo -e "${RED}✗ Integration tests failed${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Checking build warnings...${NC}"
if cargo build 2>&1 | grep -q "warning"; then
    echo -e "${YELLOW}⚠ Build has warnings${NC}"
    cargo build
else
    echo -e "${GREEN}✓ Build is clean${NC}"
fi

echo ""
echo -e "${YELLOW}Running shell tests...${NC}"
if [ -d "tests/sh" ]; then
    for test_script in tests/sh/*.sh; do
        if [ -f "$test_script" ]; then
            test_name=$(basename "$test_script" .sh)
            echo -e "  Running ${test_name}..."
            if bash "$test_script" > /tmp/test_output.txt 2>&1; then
                echo -e "  ${GREEN}✓ ${test_name} passed${NC}"
            else
                echo -e "  ${RED}✗ ${test_name} failed${NC}"
                cat /tmp/test_output.txt
                exit 1
            fi
        fi
    done
    echo -e "${GREEN}✓ All shell tests passed${NC}"
else
    echo -e "${YELLOW}No shell tests found${NC}"
fi

echo ""
echo "=================================="
echo -e "${GREEN}All tests passed!${NC}"
echo "=================================="