#pragma version(1)
#pragma rs java_package_name(com.tangzm.facedetect)
#pragma rs_fp_full

float* opMat1;
float* opMat2;
float* resultMat;

int numRow;
int numColumn;
int dim;

void __attribute__((kernel)) muliply(uint8_t in, uint32_t x, uint32_t y) {
	int i=0;
	float result = 0;
	
	for (i=0; i<dim; i++) {
		result += (opMat1[y*dim+i] * opMat2[i*numColumn+x]);
	}
	
	resultMat[x+y*numColumn] = result;
}


void __attribute__((kernel)) mulRep(uint8_t in, uint32_t x, uint32_t y) {
	int i=0;
	int j=0;
	int k=0;
	int l=0;
	float value = 0;
	
	int offset = y*dim*numColumn;
	
	for (i=0; i<numColumn; i++) {
		for (j=0; j<dim; j++) {
			value = 0;
			for (k=0; k<dim; k++) {
				value += opMat1[j*dim+k] * opMat2[offset+k*numColumn+i];
			}
			resultMat[offset+j*numColumn+i] = value;
		}
	}
}

//Using LU mat for resolve, using in-place calculation
// opMat1: the lu matrix
// resultMat: using in-place calculation. it contains the target value before computation and return the result after computation
void __attribute__((kernel)) resolve(uint8_t in, uint32_t x) {
	int i=0; 
	int j=0;
	float sum=0;
	
	//forward substitution
	//using L matrix, be notified the diagonal of L is all ones
	for (i=0; i<numRow; i++) {
		sum = 0;
		
		for (j=0; j<i; j++) {
			sum += opMat1[i*numColumn+j] * resultMat[j*dim+x];
		}
		
		resultMat[i*dim+x] = resultMat[i*dim+x]-sum;
	}
	
	//backward substitution
	//using U matrix
	for (i=numRow-1; i>=0; i--) {
		sum = 0;
		
		for (j=numRow-1; j>i; j--) {
			sum += opMat1[i*numColumn+j] * resultMat[j*dim+x];
		}
		
		resultMat[i*dim+x] = (resultMat[i*dim+x] - sum)/opMat1[i*numColumn+i];
	}
}