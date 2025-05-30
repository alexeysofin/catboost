cd ..
mkdir -p ./shap/catboost_build
python3 build/build_native.py --build-root-dir=./shap/catboost_build --targets catboostmodel --build-type Debug
