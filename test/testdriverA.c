#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>

#include "CInterfaceOxTTL.h"

#include "BasicRDFGraphComparator.h"
#include <NQuadsRDF.h>

const char* datafile = NULL;
const char* outfile = NULL;
bool expect = true;
bool use_trig = false;

static int parse_args(int argc, char *argv[]);
static TripleStream* generate_input_graph_from_trig(const char*);
static TripleStream* generate_input_graph_from_ttl(const char*);
static TripleStream* generate_test_graph(const char* filepath);
static char* load_input_into_memory(const char* filepath);

int main(int argc, char *argv[]){
	bool are_same_graphs;
	TripleStream* input_graph, *test_graph;
	int err = parse_args(argc, argv);
	if (err != 0) exit(EXIT_FAILURE);
	if (use_trig){
		input_graph = generate_input_graph_from_trig(datafile);
	} else {
		input_graph = generate_input_graph_from_ttl(datafile);
	}
	if (input_graph == NULL){
		fprintf(stderr, "Failed to load premise graph\n");
		exit(EXIT_FAILURE);
	}
	test_graph = generate_test_graph(outfile);
	if (test_graph == NULL){
		free_TripleStream(input_graph);
		fprintf(stderr, "Failed to load test graph\n");
		exit(EXIT_FAILURE);
	}

	are_same_graphs = compare_triples(input_graph, test_graph);
	if (are_same_graphs){
		fprintf(stderr, "input and expect are isomorph\n");
	} else {
		fprintf(stderr, "input and expect are not isomorph\n");
	}
	free_TripleStream(input_graph);
	free_TripleStream(test_graph);
	if (expect && !are_same_graphs){
		fprintf(stderr, "premise isnt equal to expect graph.\n");
		exit(EXIT_FAILURE);
	} else if (!expect && are_same_graphs){
		fprintf(stderr, "expected that premise and expect "
				"graph not to be equal but they are.\n");
		exit(EXIT_FAILURE);
	}
	exit(EXIT_SUCCESS);
}

static TripleStream* generate_input_graph_from_trig(const char* filepath){
	int err;
	TripleStream* ret;
	TrigConfig* config = NULL;
	char* input = load_input_into_memory(filepath);
	if (input == NULL) return NULL;
	ret = new_TripleStream();
	err = parse_trig(input, (TripleHandler*) append_TripleStream, ret, config);
	free(input);
	return ret;
}

static TripleStream* generate_input_graph_from_ttl(const char* filepath){
	int err;
	TripleStream* ret;
	TTLConfig* config = NULL;
	char* input = load_input_into_memory(filepath);
	if (input == NULL) return NULL;
	ret = new_TripleStream();
	err = parse_ttl(input, (TripleHandler*) append_TripleStream, ret, config);
	free(input);
	return ret;
}

static TripleStream* generate_test_graph(const char* filepath){
	int err;
	TripleStream* ret = new_TripleStream();
	err = nquads_parse_file(filepath,
			(TripleHandler*) append_TripleStream, ret);
	return ret;
}

static struct option parse_options[] = {
	{"premise", required_argument, NULL, 'p'},
	{"out", required_argument, NULL, 'o'},
	{"expected-failure", no_argument, NULL, 'f'},
	{"use-trig", no_argument, NULL, 'x'},
        {NULL, 0, NULL, 0}
};

static int parse_args(int argc, char *argv[]){
	int err = 0;
	int c = 0;
	int option_index;
	while(c != -1){
		c = getopt_long(argc, argv, "",
				parse_options, &option_index);
		switch(c){
			case -1: //end of arguments
				break;
			case 'x':
				use_trig = true;
				break;
			case 'f':
				expect = false;
				break;
			case 'p':
				datafile = optarg;
				break;
			case 'o':
				outfile = optarg;
				break;
			default:
				fprintf(stderr, "unrecognized argument %c\n", c);
				err = 1;
				break;
		}
	}
	return err;
}


static char* load_input_into_memory(const char* filepath){
	char *ret;
	long fsize;
	FILE *f = fopen(filepath, "rb");
	if (f == NULL){
		fprintf(stderr, "Failed to open file %s\n", filepath);
		return NULL;
	}
	fseek(f, 0, SEEK_END);
	fsize = ftell(f);
	rewind(f);
	//fseek(f, 0, SEEK_SET);  /* same as rewind(f); */

	ret = malloc(fsize + 1);
	fread(ret, fsize, 1, f);
	ret[fsize] = 0;
	fclose(f);
	return ret;
}
