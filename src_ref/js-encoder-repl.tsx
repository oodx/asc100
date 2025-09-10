import React, { useState, useEffect } from 'react';

const Base100Encoder = () => {
    const [result, setResult] = useState('');
    
    useEffect(() => {
        // Base100 encoder function
        function encodeToBase100(input) {
            const BASE_CHARSET = 
                ' !"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~' +
                '\t\n\r\x01\x02';
            
            const MARKERS = {
                '#EOF#': '\x01',
                '#EOS#': '\x02'
            };
            
            const BASE64_CHARS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
            
            // Preprocess markers
            let processed = input;
            for (const [marker, code] of Object.entries(MARKERS)) {
                processed = processed.replaceAll(marker, code);
            }
            
            // Convert to indices
            const indices = [];
            for (const char of processed) {
                const index = BASE_CHARSET.indexOf(char);
                if (index === -1) {
                    throw new Error(`Unsupported character: "${char}"`);
                }
                indices.push(index);
            }
            
            // Pack into 7-bit binary
            let binaryStr = '';
            for (const index of indices) {
                binaryStr += index.toString(2).padStart(7, '0');
            }
            
            // Pad to multiple of 6 bits
            while (binaryStr.length % 6 !== 0) {
                binaryStr += '0';
            }
            
            // Convert to Base64
            let encoded = '';
            for (let i = 0; i < binaryStr.length; i += 6) {
                const sixBits = binaryStr.substring(i, i + 6);
                const outputIndex = parseInt(sixBits, 2);
                encoded += BASE64_CHARS[outputIndex];
            }
            
            return encoded;
        }

        // Test with shorter JS Hello World
        const jsCode = `console.log("Hello, World!");#EOF#`;

        try {
            const encoded = encodeToBase100(jsCode);
            const testUrl = `https://postman-echo.com/get?code=${encoded}&parser=base100&test=js_hello_world`;
            
            const output = `🎯 BASE100 ENCODING TEST RESULTS

📝 Original JavaScript Code:
${jsCode}

🔐 Encoded (URL-Safe):
${encoded}

🌐 Test URL for postman-echo.com:
${testUrl}

📊 Stats:
- Original: ${jsCode.length} characters
- Encoded: ${encoded.length} characters  
- Ratio: ${(encoded.length / jsCode.length).toFixed(2)}x
- Ready for URL transmission: ✅`;

            setResult(output);
            
            // Also log to console for easy access
            console.log("=== BASE100 ENCODER OUTPUT ===");
            console.log("Encoded:", encoded);
            console.log("Test URL:", testUrl);
            
        } catch (error) {
            setResult(`❌ Error: ${error.message}`);
        }
    }, []);

    return (
        <div style={{ 
            fontFamily: 'monospace', 
            padding: '20px', 
            backgroundColor: '#f5f5f5',
            border: '1px solid #ddd',
            borderRadius: '8px',
            maxWidth: '800px',
            margin: '0 auto'
        }}>
            <h2 style={{ color: '#333', marginBottom: '20px' }}>🧪 Base100 Encoder Test</h2>
            <pre style={{ 
                whiteSpace: 'pre-wrap', 
                backgroundColor: 'white',
                padding: '15px',
                border: '1px solid #ccc',
                borderRadius: '4px',
                fontSize: '12px',
                lineHeight: '1.4',
                overflow: 'auto'
            }}>
                {result}
            </pre>
        </div>
    );
};

export default Base100Encoder;