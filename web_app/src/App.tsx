import { useState, useEffect, useCallback } from 'react';
// @ts-ignore - will be available after build
import init, { encrypt_file, decrypt_file, init_panic_hook } from './pkg/file_encryptor_wasm';
import { Upload, Lock, Unlock, FileText, Download, Loader2, ShieldCheck, RefreshCw } from 'lucide-react';
import { useDropzone } from 'react-dropzone';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export default function App() {
  const [isWasmReady, setIsWasmReady] = useState(false);
  const [mode, setMode] = useState<'encrypt' | 'decrypt'>('encrypt');
  const [file, setFile] = useState<File | null>(null);
  const [password, setPassword] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [result, setResult] = useState<{ url: string; name: string } | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    init().then(() => {
      init_panic_hook();
      setIsWasmReady(true);
      console.log("WASM Initialized");
    }).catch((e: any) => {
      console.error("WASM Init Failed", e);
      setError("Failed to initialize encryption engine.");
    });
  }, []);

  const onDrop = useCallback((acceptedFiles: File[]) => {
    if (acceptedFiles.length > 0) {
      setFile(acceptedFiles[0]);
      setResult(null);
      setError(null);

      // Auto-detect mode based on extension
      if (acceptedFiles[0].name.endsWith('.encrypted')) {
        setMode('decrypt');
      }
    }
  }, []);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({ onDrop, maxFiles: 1 });

  const handleProcess = async () => {
    if (!file || !password) return;
    setIsProcessing(true);
    setError(null);
    setResult(null);

    try {
      const buffer = await file.arrayBuffer();
      const bytes = new Uint8Array(buffer);

      let processedBytes: Uint8Array;
      let outName = file.name;

      if (mode === 'encrypt') {
        processedBytes = encrypt_file(password, bytes);
        outName = `${file.name}.encrypted`;
      } else {
        processedBytes = decrypt_file(password, bytes);
        if (outName.endsWith('.encrypted')) {
          outName = outName.slice(0, -10); // remove .encrypted
        } else {
          outName = `${outName}.decrypted`;
        }
      }

      const blob = new Blob([processedBytes as any], { type: 'application/octet-stream' });
      const url = URL.createObjectURL(blob);
      setResult({ url, name: outName });
    } catch (e: any) {
      console.error(e);
      setError(e.toString() || "An error occurred during processing.");
    } finally {
      setIsProcessing(false);
    }
  };

  const reset = () => {
    setFile(null);
    setResult(null);
    setPassword('');
    setError(null);
  };

  if (!isWasmReady) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center text-white">
        <Loader2 className="w-8 h-8 animate-spin text-primary-500" />
        <span className="ml-3">Loading Security Engine...</span>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-dark-bg text-slate-200 p-4 md:p-8 flex flex-col items-center justify-center relative overflow-hidden">
      {/* Background Gradients */}
      <div className="absolute top-0 left-0 w-full h-full overflow-hidden -z-10 pointer-events-none">
        <div className="absolute top-[-20%] right-[-10%] w-[500px] h-[500px] rounded-full bg-primary-900/20 blur-[100px]" />
        <div className="absolute bottom-[-20%] left-[-10%] w-[600px] h-[600px] rounded-full bg-blue-900/10 blur-[120px]" />
      </div>

      <div className="w-full max-w-xl">
        <div className="text-center mb-10">
          <div className="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-primary-600/20 mb-4 ring-1 ring-primary-500/30">
            <ShieldCheck className="w-8 h-8 text-primary-400" />
          </div>
          <h1 className="text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-primary-400 to-blue-200 mb-2">
            Secure File Vault
          </h1>
          <p className="text-slate-400">
            Client-side encryption. Your data never leaves your device.
          </p>
        </div>

        <div className="bg-dark-surface border border-dark-border rounded-2xl shadow-2xl overflow-hidden backdrop-blur-sm">
          {/* Tabs */}
          <div className="flex border-b border-dark-border">
            <button
              onClick={() => setMode('encrypt')}
              className={cn(
                "flex-1 py-4 font-medium text-sm transition-colors flex items-center justify-center gap-2",
                mode === 'encrypt'
                  ? "text-primary-400 bg-primary-500/5 border-b-2 border-primary-500"
                  : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
              )}
            >
              <Lock className="w-4 h-4" /> Encrypt
            </button>
            <button
              onClick={() => setMode('decrypt')}
              className={cn(
                "flex-1 py-4 font-medium text-sm transition-colors flex items-center justify-center gap-2",
                mode === 'decrypt'
                  ? "text-primary-400 bg-primary-500/5 border-b-2 border-primary-500"
                  : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
              )}
            >
              <Unlock className="w-4 h-4" /> Decrypt
            </button>
          </div>

          <div className="p-6 md:p-8 space-y-6">
            {/* File Dropzone */}
            {!file ? (
              <div
                {...getRootProps()}
                className={cn(
                  "border-2 border-dashed rounded-xl h-64 flex flex-col items-center justify-center cursor-pointer transition-all duration-300 group",
                  isDragActive
                    ? "border-primary-500 bg-primary-500/10 scale-[0.99]"
                    : "border-dark-border hover:border-primary-500/50 hover:bg-white/5"
                )}
              >
                <input {...getInputProps()} />
                <div className="w-16 h-16 rounded-full bg-slate-800/50 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
                  <Upload className="w-8 h-8 text-slate-400 group-hover:text-primary-400" />
                </div>
                <p className="text-lg font-medium text-slate-300">
                  {isDragActive ? "Drop file now" : "Drag & drop your file here"}
                </p>
                <p className="text-sm text-slate-500 mt-2">
                  or click to browse
                </p>
              </div>
            ) : (
              <div className="bg-slate-800/50 border border-dark-border rounded-xl p-4 flex items-center justify-between">
                <div className="flex items-center gap-3 overflow-hidden">
                  <div className="w-10 h-10 rounded-lg bg-primary-600/20 flex items-center justify-center flex-shrink-0">
                    <FileText className="w-5 h-5 text-primary-400" />
                  </div>
                  <div className="min-w-0">
                    <p className="font-medium text-slate-200 truncate">{file.name}</p>
                    <p className="text-xs text-slate-500">{(file.size / 1024).toFixed(2)} KB</p>
                  </div>
                </div>
                <button
                  onClick={reset}
                  className="p-2 hover:bg-white/10 rounded-lg text-slate-400 hover:text-slate-200 transition-colors"
                  aria-label="Remove file"
                >
                  <RefreshCw className="w-4 h-4" />
                </button>
              </div>
            )}

            {/* Password Input */}
            <div>
              <label className="block text-sm font-medium text-slate-400 mb-2">
                Encryption Password
              </label>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="Enter a strong password..."
                className="w-full bg-dark-bg border border-dark-border rounded-lg px-4 py-3 text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary-500/50 focus:border-primary-500 transition-all placeholder:text-slate-600"
              />
            </div>

            {/* Error Message */}
            {error && (
              <div className="bg-red-500/10 border border-red-500/20 rounded-lg p-3 text-sm text-red-400">
                {error}
              </div>
            )}

            {/* Action Button */}
            {!result ? (
              <button
                onClick={handleProcess}
                disabled={!file || !password || isProcessing}
                className={cn(
                  "w-full py-3.5 rounded-lg font-semibold text-white shadow-lg shadow-primary-500/20 transition-all flex items-center justify-center gap-2",
                  !file || !password || isProcessing
                    ? "bg-slate-700 text-slate-400 cursor-not-allowed"
                    : "bg-gradient-to-r from-primary-600 to-primary-500 hover:from-primary-500 hover:to-primary-400 transform hover:-translate-y-0.5"
                )}
              >
                {isProcessing ? (
                  <>
                    <Loader2 className="w-5 h-5 animate-spin" /> Processing...
                  </>
                ) : (
                  <>
                    {mode === 'encrypt' ? <Lock className="w-4 h-4" /> : <Unlock className="w-4 h-4" />}
                    {mode === 'encrypt' ? 'Encrypt File' : 'Decrypt File'}
                  </>
                )}
              </button>
            ) : (
              <a
                href={result.url}
                download={result.name}
                className="block w-full"
              >
                <button className="w-full py-3.5 rounded-lg font-semibold text-white bg-green-600 hover:bg-green-500 shadow-lg shadow-green-500/20 transition-all flex items-center justify-center gap-2 transform hover:-translate-y-0.5">
                  <Download className="w-5 h-5" /> Download Result
                </button>
              </a>
            )}
          </div>
        </div>

        <div className="mt-8 text-center text-slate-600 text-sm">
          <p>Powered by Rust & WebAssembly</p>
        </div>
      </div>
    </div>
  );
}
